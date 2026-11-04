# A permanent Codex client for testing Fetch

This setup gives you a separate Codex client that connects directly to Fetch only.
It uses the Codex program already installed on the Mac. Your normal Codex settings,
servers and saved logins stay where they are.

The test client's settings, sessions and new logins persist in:

```text
~/Library/Application Support/Fetch-Development/clients/codex-test/
```

The `fetch_codex` command opens it again after closing Terminal or restarting the
Mac. The command also accepts Codex arguments, including `login` and `mcp login
fetch`. Fetch can expose tools from the servers managed inside Fetch; those tools
are part of the Fetch connection you are testing.

## Set up each Mac once

Install Codex CLI and Python 3 first. Confirm `codex --version` and
`python3 --version` work. This setup expects the installed Codex to support
`--no-daemon` and the `apps`, `plugins` and `remote_plugin` feature switches.
The original setup was checked with Codex CLI 0.166.0.

Development Fetch uses `https://localhost:53672/mcp` and its public certificate at
`~/Library/Application Support/Fetch-Development/certificate.pem`. Complete the
Mac's development Fetch setup in [DEVELOPMENT.md](DEVELOPMENT.md). That certificate
must exist before connecting. The launcher supplies it to this Codex process;
it does not disable HTTPS certificate checks or change system trust.

Run the following in Terminal. It creates the test folder and a small command at
`~/.local/bin/fetch_codex`. It refuses to overwrite either if already present.
If you already have this setup, skip creation and use the commands below.

```sh
python3 - <<'PY'
from pathlib import Path
import json
import os

os.umask(0o077)
root = Path.home() / 'Library/Application Support/Fetch-Development/clients/codex-test'
command = Path.home() / '.local/bin/fetch_codex'
for path in (root, command):
    if path.exists() or path.is_symlink():
        raise SystemExit(f'Already exists: {path}. No changes made; reuse the existing setup.')

root.mkdir(mode=0o700, parents=True)
(root / 'sqlite').mkdir(mode=0o700)
(root / 'workspace').mkdir(mode=0o700)
command.parent.mkdir(mode=0o700, parents=True, exist_ok=True)

# Record the MCP definition first, then generate this client's Codex settings.
source = {'mcpServers': {'fetch': {'url': 'https://localhost:53672/mcp'}}}
with (root / '.mcp.json').open('x') as file:
    json.dump(source, file, indent=2)
    file.write('\n')
url = json.loads((root / '.mcp.json').read_text())['mcpServers']['fetch']['url']
config = '''# Generated from this directory's .mcp.json for the dedicated Fetch test client.
cli_auth_credentials_store = "file"
mcp_oauth_credentials_store = "file"

[features]
apps = false
plugins = false
remote_plugin = false

[mcp_servers.fetch]
url = ''' + json.dumps(url) + '\n'
with (root / 'config.toml').open('x') as file:
    file.write(config)

launcher = '''#!/bin/sh
set -eu
umask 077
fetch_test_root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
fetch_codex_bin=$(command -v codex || true)
if [ -z "$fetch_codex_bin" ]; then
  fetch_codex_bin="$HOME/.local/bin/codex"
fi
cd "$fetch_test_root/workspace"
export CODEX_HOME="$fetch_test_root"
export CODEX_SQLITE_HOME="$fetch_test_root/sqlite"
export CODEX_CA_CERTIFICATE="$HOME/Library/Application Support/Fetch-Development/certificate.pem"
exec "$fetch_codex_bin" --no-daemon "$@"
'''
with (root / 'launch.command').open('x') as file:
    file.write(launcher)
(root / 'launch.command').chmod(0o700)
with command.open('x') as file:
    file.write('''#!/bin/sh
exec "$HOME/Library/Application Support/Fetch-Development/clients/codex-test/launch.command" "$@"
''')
command.chmod(0o700)
print(f'Created {root}')
print(f'Command: {command}')
PY
```

New folders and launchers are private to your account (`0700`); new settings files
are private too (`0600`). Existing parent-directory permissions are preserved.
The setup does not edit shell settings, register global MCP servers or copy any
credentials. Its `.mcp.json` is the source of the test client's Fetch definition;
`config.toml` is generated from it locally, without running the global MCP sync.

## Sign in and use it

Open a **new Terminal tab**, especially if an earlier temporary setup defined a
shell function named `fetch_codex`. That function would otherwise hide the new
command.

Run these one at a time:

```sh
# Sign this separate client into your OpenAI account.
fetch_codex login

# Check the configured MCP list: only fetch should appear.
fetch_codex mcp list

# With development Fetch running, authorize this client to use it.
fetch_codex mcp login fetch

# Open the test Codex session.
fetch_codex
```

Inside Codex, enter `/mcp` and confirm Fetch is the only server shown. If another
server appears, resolve that configuration before using this as a Fetch-only
test. A new user-level settings folder does not override every possible
machine-managed setting.

If Terminal says `fetch_codex` is not found, use its full path with the same
arguments. No shell settings need changing:

```sh
"$HOME/.local/bin/fetch_codex" login
"$HOME/.local/bin/fetch_codex" mcp list
"$HOME/.local/bin/fetch_codex" mcp login fetch
"$HOME/.local/bin/fetch_codex"
```

You can also double-click the permanent folder's `launch.command` to open the
client. It finds Codex on the command search path, or at `~/.local/bin/codex`.

## What persists and what must be set up separately

Settings, sessions and new file-based logins stay in the permanent test folder.
Apps and plugins are disabled, the workspace starts empty, and `--no-daemon`
prevents this launcher from using the shared Codex background server.

On another Mac, recreate the setup and sign in there. Do not copy existing
Codex authentication files, plugins, hooks, provider credentials, Keychain items
or encrypted Fetch state between Macs as part of this setup. Fetch's development
signing identity and authentication are local setup too; follow its development
guide on that Mac.

This separates Codex settings and saved state. It does **not** restrict what an
MCP server can read, change or control on the computer. A live test still needs
the project's required isolated environment and test accounts. Creating these
files or listing configured servers does not prove a Fetch connection or tool
call works. This client also uses the running Fetch instance's existing managed
servers and provider logins; a fresh Codex login does not reset Fetch.

The supported locations and certificate variable are documented in
[OpenAI's environment-variable reference](https://learn.chatgpt.com/docs/config-file/environment-variables).
The feature switches and file-based credential settings are documented in
[OpenAI's configuration reference](https://learn.chatgpt.com/docs/config-file/config-reference).
