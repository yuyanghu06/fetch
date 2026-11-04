# Helper credential bridge fixtures

These fixtures use disposable keys and fake account values. They do not contact
Google or run an installed provider package.

The three `.mjs` files are synthetic login, standalone runtime and MCP-protocol
peers. The unit tests build a fake OAuth library around them, then run Fetch's
actual bootstrap with only its fixture library hash replaced through a test hook.
They are not copies of Google's implementation or proof of a real Google login.

`support.rs` stages only the public Node runtime into a temporary directory and
runs fixture processes through macOS `sandbox-exec`. Each child receives an empty
environment with a fresh home. The deny-default policy permits owned file access
and system runtime reads, executable mapping and a root-directory read needed by
the sandbox launcher. It denies external file contents, TCP and Keychain service
access. Its only network exception is the specific owned Unix socket used for
the credential bridge. Disposable read/write/TCP canaries verify confinement first.

Temporary state is owned by Rust `TempDir`; child commands use kill-on-drop.
`FETCH_HELPER_FIXTURE_NODE` may select a staged public Node24 binary. Otherwise
the test copies the existing Fetch development Node24.22.1 binary read-only.
Neither path downloads, installs or modifies a runtime.
