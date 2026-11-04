## November 4 private program folders

program_paths.rs owns the installer’s known executable folders: npm node_modules/.bin, Python venv/bin and the already prepared Railway helper’s railway-cli/node_modules/.bin. New records save these relative paths privately as _installation.program_directories; legacy rows resolve the same layout in memory. Only existing folders contained by the instance’s package enter startup. Nothing searches recursively or borrows another instance’s folders. Independent 60 checks including 10 new and locked native build pass; Evaluator ACCEPT. Real account authentication is untested.

## November 3 current wrapper records

Preparation now writes required installation/launch records without obsolete
Fetch provider-store status arrays. Original source secrets are still discarded,
records reused by original identity and no native login is guessed during install.
Managed Connect remains separate from authentication proof. Removal preserves
installed packages because they may contain native server account/config files.
Explicit internal native_setup metadata is validated by storage, but normal imports
have no new public metadata field or automatic command/path detection. Existing
Railway extra-CLI preparation is retained unchanged pending explicit compatibility
review; no new server-specific authentication code was added.

# Server preparation

Deletion holds an idle installation guard through cleanup, rejecting active preparation and excluding new source-reuse/autoconnect submissions. Removing the saved association makes a fresh discovery selectable; original definitions, scripts and shared runtimes are preserved.

D83 managed installation adds an awaited per-submission connection hook after preparation. Manager supplies a weak reference and invokes the existing named Restart / Connect operation. Successful connections remain running; auth/start failures do not undo package preparation or stop later rows. Existing ready imports remain untouched. The standalone submit helper remains preparation-only. Installation progress stays pending through the managed connection attempt; GET servers owns runtime/authentication outcomes. This source has not been built or run in this pass.

`mod.rs` captures selected source entries without retaining credential values. It keeps one installation group, prepares entries sequentially, reuses saved successful imports and retains progress for the current run. Packages and runtimes belong under Fetch's private Application Support root; durable server records remain separate.

Source includes pinned Node 24.22.1, uv 0.13.5 and Python 3.13.18 provisioning, HTTPS download limits, published SHA-256 comparison, private preparation homes/caches and isolated npm/Python environments. No runtime setup or package command was executed during implementation. Fresh-install behavior and all failure/cleanup paths await user-controlled testing.

Primary references: [Node distribution](https://nodejs.org/download/release/latest-v24.x/), [Python release](https://blog.python.org/), [uv release](https://github.com/astral-sh/uv/releases/tag/0.13.5), [uv storage](https://docs.astral.sh/uv/reference/storage/) and [managed Python](https://docs.astral.sh/uv/guides/install-python/). No third-party implementation was copied.

The injected root now defaults to ~/Library/Application Support/Fetch-Development
for native debug builds and ~/Library/Application Support/Fetch for release.
packages/servers, packages/runtimes and packages/cache stay below that root;
existing local scripts remain in place and remote services remain remote.
No installation was performed to verify these paths in this change.

October 29 import follow-up: recognized `mcp-remote` source forms are normalized
by discovery into native remote records. Preparation saves those fixed endpoints
and fresh header credential references without installing the bridge or copying
source header/environment values. The shared saved-record validator admits the
same narrow public endpoint/query profiles as discovery. A repeated successful
source path/name reuses the existing record and preserves its saved destination
and Fetch settings even if the source definition later changes. Disposable
fixture coverage exercises this without package downloads or connections.

Official @railway/mcp-server imports also prepare pinned @railway/cli5.71.0 in
a separate railway-cli dependency folder under their private installation. This
provides the executable required by the MCP shim without using personal PATH or
modifying the shim's installed dependency tree during repair. Shared startup now adds the existing private CLI folder for new and older installations; startup/status do not install missing programs. Native binary
existence/permissions/path containment and pinned package metadata gate readiness;
no CLI --version is run. npm's official postinstall fetches its binary over HTTPS;
it does not provide an independent archive checksum. No package install executed
in this source pass. First-party download HTTP now uses safe diagnostics.


November3 source walkthrough confirmed the existing HTTP/MCP product path uses
Manager's connection hook; raw Installer::submit remains preparation-only. The
original request contains source path/name selections; the later approved direct
inputs are described below and still do not accept credential values. Preparation errors currently collapse to INSTALLATION_FAILED;
connection/login outcomes live in GET servers and do not undo installation.
This was read-only inspection, not a new installation or runtime test.


Direct POST bodies also accept server_name plus command or url. direct.rs checks
literal command syntax before shlex splitting, builds an ordinary shared capture
and computes a digest of the name/normalized original definition. Existing
private npm/Python/script/remote preparers and managed Connect hooks handle it.
Repeated direct identity reuses saved settings/credentials even after edits;
source-path/name reuse remains separate. Direct progress uses config_path:null
and never echoes command/URL text. Inputs carry no credential values; no shell
or documentation interpreter is used. No UI is added for these new inputs.
