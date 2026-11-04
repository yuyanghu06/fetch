# Python MCP setup on another Mac

Fetch installs Python package servers into its own folders. You do not need to
install a global Python or uv just to install a supported `uvx` MCP server.
Fetch downloads its managed uv installer, and uv installs compatible Python and
the server's dependencies. Each server installation gets a separate package
environment and separate account files.

This behavior applies to **new Python package installations imported from uvx
commands**. Existing installed servers keep their saved executable and settings.
Standalone local Python script imports still use the older provisioning path;
this change does not replace that path.

## Set up Fetch on the Mac

1. Use the current Fetch source on that Mac. Read [DEVELOPMENT.md](DEVELOPMENT.md)
   and the first-development-launch checklist in [AGENTS.md](../../AGENTS.md).
2. Check Rust against the `rust-version` in Cargo.toml, Apple's command-line
   developer tools, and `/usr/bin/codesign`. Install missing tools using the
   existing checklist; the user handles installers and macOS approvals.
3. Select a development signing identity that exists **on that Mac**, including
   its private key. The launcher reads `FETCH_DEV_SIGNING_IDENTITY` first, then
   `~/Library/Application Support/Fetch-Development/development-signing-identity`.
   Do not copy a fingerprint from another Mac and assume its certificate exists.
   Follow the existing certificate setup instructions; leave trust unchanged.
4. Build and verify the signed development executable:

   ```sh
   ./scripts/run-development.sh --build-only
   ```

5. If Fetch is already running, explicitly **Quit** it. Closing its window only
   backgrounds it. Then launch the newly built version:

   ```sh
   ./scripts/run-development.sh
   ```

No global Python version, shell-profile change or Python activation command is
needed for the supported Python package install path. The Mac needs network
access to download uv, managed Python and packages from their upstream sources.

## Install a Python package server

In Fetch's Install screen, choose the configuration file or folder on **this
Mac**, then select the server and install it. Selecting `.codex` reads that
folder's `config.toml` only. A command such as `uvx blender-mcp` needs no fixed
Python version added by Fetch.

If the server's supplied launch instructions explicitly request Python, Fetch
preserves supported forms such as:

```text
uvx --python 3.12 blender-mcp
uvx --python=3.12 blender-mcp
```

These are examples of imported launch commands, not a requirement to run them
manually. With no Python request, uv handles compatible interpreter selection.
With a request, uv keeps it as a constraint. Fetch does not force Python 3.13.18
for these package installs or implement a second Python-version resolver.

Package version requests and already supported server arguments are preserved.
Fetch records the actual installed package version and executable, so normal
startup does not update the package. uv manages Python and dependencies; Fetch
still owns its normal preparation, process and permission rules.

If changing a user-installed MCP definition managed by the shared configuration,
update `~/.config/shared-mcp/servers.json` first, then run its `sync.py` and
`sync.py --check`. Do not independently edit generated client MCP entries. This
guide does not require changing any existing definition.

## Where files live

Paths use the current Mac user's home directory:

| Files | Development path |
| --- | --- |
| Managed uv and reusable Python installations | `~/Library/Application Support/Fetch-Development/packages/runtimes/` |
| One server's uv environments | `<root>/packages/servers/<installation-id>/tools/` |
| That installation's exported commands | `<root>/packages/servers/<installation-id>/bin/` |
| Download/preparation cache | `<root>/packages/cache/` |
| Per-server account/configuration files | `<root>/servers/<server-directory>/` |

Here `<root>` is `~/Library/Application Support/Fetch-Development` for an ordinary
debug build, and `~/Library/Application Support/Fetch` for a release build.
Explicit data-directory overrides still apply. uv may reuse a compatible Python
installation across several environments; their installed packages remain
separate. Older saved installations may still use their existing `venv/bin`.

On another Mac, install the servers afresh and sign into accounts when requested.
Do not copy Python environments, Fetch state, credentials or private signing keys
as a setup shortcut. Executable paths and signing setup are local to each Mac;
existing encrypted files alone do not provide their Keychain keys. Existing
local-script imports also need their original script paths to exist on that Mac.

## What has been verified

32 focused checks and the signed native development build pass. The new installer
check uses an OS-confined uv fixture to exercise normal Blender import, explicit
Python selection, separate installations, saved package metadata and executable
folders. Independent source review accepts the change.

A real Blender package installation and Blender connection have **not** been
verified for this change. A planned restricted download experiment stopped
before uv ran because macOS rejected the requested network sandbox rule; its
state was cleaned up. New MCP connection tests must still use the isolated test
environment required by AGENTS.md. A Python environment separates dependencies;
it does not itself isolate a server from the user's files or desktop apps.

The existing generic installation error also does not retain uv's original error
output. Do not describe a generic failure as proof of a specific package or
Python error without additional evidence.

Implementation/evidence: [milestone](../../milestones/python-server-environments/info.md).
Upstream behavior: [uv tool environments](https://docs.astral.sh/uv/concepts/tools/),
[Python management](https://docs.astral.sh/uv/concepts/python-versions/), and
[uv storage controls](https://docs.astral.sh/uv/reference/environment/).
