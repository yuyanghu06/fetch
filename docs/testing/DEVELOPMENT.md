# Development state and a fresh Codex client

October 28, 2026. The user selected separate development/final-build state,
portable paths resolved from the current user's home, import of existing global
Codex definitions and a fresh Codex client connected only to Fetch. The main
Codex client can connect to the same running development Fetch instance.

## Paths on either Mac

`~` below means the current user's home, not a fixed username. Code resolves HOME
at launch; it does not embed the developer's path. First native launch creates
missing private Fetch directories and fresh identity. Package installation and
provider authentication still require their normal actions. Copy source to the
Mac Mini and initialize there; do not copy encrypted credentials/Keychain state
or old machine-specific saved installations.

| State | Ordinary debug build | Final release build |
| --- | --- | --- |
| Fetch root | `~/Library/Application Support/Fetch-Development/` | `~/Library/Application Support/Fetch/` |
| Installed MCP packages | `<root>/packages/servers/` | `<root>/packages/servers/` |
| Private Node/Python/uv runtimes | `<root>/packages/runtimes/` | `<root>/packages/runtimes/` |
| Preparation caches | `<root>/packages/cache/` | `<root>/packages/cache/` |
| Saved config, encrypted credentials and default server homes | `<root>/servers/` | `<root>/servers/` |
| Encrypted app authorization/identity | `<root>/application/` | `<root>/application/` |
| Public TLS certificate | `<root>/certificate.pem` | `<root>/certificate.pem` |
| MCP address | `https://localhost:53672/mcp` | `https://localhost:53671/mcp` |

Keychain items remain in the current user's login Keychain, with distinct service
names derived from the selected canonical root. They are not files inside this
folder. No previous data or credentials are moved automatically. Old builds may
have created state under Fetch; preserve it until deliberately reviewed.

Development means debug assertions are enabled, as in normal `cargo run`.
Release means they are disabled, as in normal `cargo run --release`. Explicit
`--data-dir` and `--port` override these defaults.

## Launching

From the checkout, the normal native development command is:

```sh
cargo run --locked --bin fetch -- app
```

To make the same development state choice explicit, including when testing an
optimized/release binary:

```sh
cargo run --locked --release --bin fetch -- app \
  --data-dir "$HOME/Library/Application Support/Fetch-Development" \
  --port 53672
```

Without those overrides, a release build uses final Fetch state and port 53671.
The original `dev` subcommand is only the older fixture backend: no native window,
no app grants or managed-server startup, `.fetch-dev` state by default and port
53671. It is different from the native debug app.

An already-running binary keeps its old behavior. Explicitly Quit it and rebuild
before relying on these defaults; closing its window only backgrounds it. Never
stop another service automatically to clear a port conflict.

## Global import and two Codex clients

1. Launch development Fetch with its separate root while leaving discovery on the
   normal current home/global Codex configuration. By default this is
   `~/.codex/config.toml`; an existing valid CODEX_HOME changes the source root.
   In Discover and install, use **Choose file…** beside the optional path field
   to select that file in the macOS picker, which shows hidden folders such as
   `.codex`. Selection fills an absolute path; Cancel preserves the current field.
   Picking alone does not discover or install servers. The field also remains
   editable and expects an expanded absolute path, not literal `~` or `$HOME`.
   Obtain the correct path on either Mac with `printf '%s\n' "$HOME/.codex/config.toml"`.
2. Select supported definitions and install them through Fetch. Import reads
   definitions, does not modify the global config and discards source secret
   values. Supported package servers get development-owned installs; existing
   local-script entries still reference their original scripts. Remote entries
   connect to the original service. Unsupported command/config shapes stay
   unsupported instead of being guessed.
3. Prepare a fresh Codex state folder at
   `~/Library/Application Support/Fetch-Development/clients/codex/`, with only the
   Fetch-development MCP entry. Its disposable workspace can live at
   `~/Library/Application Support/Fetch-Development/workspaces/codex/`.
   Configure the test profile to use file-based MCP OAuth credential storage
   (`mcp_oauth_credentials_store = "file"`) and verify that its store starts empty,
   rather than relying on an existing login-Keychain entry. This is a supported
   [Codex setting](https://learn.chatgpt.com/docs/config-file/config-reference).
   Apply CODEX_HOME only to this test Codex process, not to the Fetch process
   importing normal definitions. Do not copy everyday tokens, plugins or hooks.
4. Authenticate the test Codex client to development Fetch. The main Codex can
   also connect to that same endpoint. Each has its own Fetch authorization and
   can be revoked separately; only one Fetch instance and one set of its managed
   servers are needed. Configuring the main client must follow the existing
   `~/.config/shared-mcp/servers.json` source-first sync workflow. Do not hand-edit
   generated personal MCP settings. Before writing a test profile, record its
   definition in an authoritative source and choose a scoped generation step
   that cannot overwrite everyday settings; no profile was created in this pass.
5. Before login, configure and verify normal TLS trust for the development
   certificate in the test client. Codex documents `CODEX_CA_CERTIFICATE` for a
   private PEM trust bundle; check installed-client support and keep the setting
   scoped to this test process. Do not disable certificate validation or silently
   change system trust. Then verify the fresh client's server list contains only Fetch, complete approval,
   list its visible tools and make one bounded call. Also confirm the main
   client's existing authorization still works. This is a client-authentication
   test, separate from testing fresh provider login inside Fetch.

For the already-prepared test profile and workspace, scope the new home to its
process rather than exporting it for the whole shell:

```sh
env CODEX_HOME="$HOME/Library/Application Support/Fetch-Development/clients/codex" \
  codex --cd "$HOME/Library/Application Support/Fetch-Development/workspaces/codex"
```

The directories must exist first. Inspect effective configuration for inherited
project/system MCP entries or plugins before claiming Fetch is its only server.
Official sources: [Codex environment variables](https://learn.chatgpt.com/docs/config-file/environment-variables),
[configuration layers](https://learn.chatgpt.com/docs/config-file/config-advanced),
and [MCP configuration/login](https://learn.chatgpt.com/docs/extend/mcp).

Both Codex clients share that Fetch instance's installed servers, configuration
and provider logins. A fresh Codex profile does not reset Fetch's provider login.
Separate folders prevent state collisions; they do not restrict MCP file access
or create separate provider accounts. Explicit imported working directories stay
unchanged; otherwise stdio servers now run in their private server home rather
than inheriting the checkout directory. Preserve the existing protection and
short-test requirements when selecting actual connections.

## Evidence

The path/default/cwd changes received independent source review. The command
`cargo build --locked --bin fetch --message-format short` then succeeded with
exit 0 in 9.74 seconds. Four classic-Keychain deprecation warnings and one unread
protocol field warning remain. Four regression tests were added in
`tests/environment_paths.rs` but not executed. No native app, Keychain, MCP/client
login, installation, global-config change, certificate-trust action or release
was performed by agents. Actual authentication and installation results remain pending.

### Config-file picker follow-up

The Choose file… change compiled successfully with the locked native binary
command (exit0,11.35seconds) and received independent source acceptance. The
picker itself was not opened by agents. After relaunch, check selecting a hidden
Codex config, Cancel preserving input, repeated clicks, and closing/reopening or
quitting while the sheet is open. No new test suite was added for this UI control.

### Discovery list and current unsupported entries

The Install MCPs list has its own bounded scroll area and an always-visible
Install selected servers footer. Empty selection, busy operations, an open picker
or an active install disable the button rather than removing it. Progress details
belong in scrolling content. Manual long-list/minimum-window interaction still
needs verification after rebuilding and relaunching.

Read-only inspection of the current global Codex entry shapes found these import
limits, without running servers or exposing configuration secrets:

| Entry | Current reason |
| --- | --- |
| Hugging Face | Its URL includes a query string; Fetch currently rejects URL queries. |
| Miro | Its root endpoint is outside the recognized import path profiles. |
| Vercel | Its root endpoint is outside the recognized import path profiles. |
| v0 | Its npx command uses mcp-remote with URL/header arguments, beyond the currently supported package-launch grammar. |

These labels describe importer limitations, not failed server/network tests or
evidence that the services are broken. This layout change does not broaden the
importer or remove authentication arguments. Source credentials remain private.

Select all is in the fixed toolbar above the list and affects supported entries
only. Supported entries disabled in the source can be explicitly selected by
this bulk action; original source flags are unchanged. The footer includes the
selected count. The locked native binary build passed (exit0,10.45seconds) after
these layout/bulk-selection edits; actual running-window checks remain pending.
