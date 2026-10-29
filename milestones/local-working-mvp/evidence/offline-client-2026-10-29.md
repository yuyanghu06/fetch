# Separate offline Codex preparation

Root: `/Users/yuyang/Library/Application Support/Fetch-Development/local-mvp-proof/`.
All created files are private and test-owned. The Codex executable was copied
from the existing installation; no existing configuration, auth, hooks, plugins,
SQLite state or credentials were copied or modified. Installed binary unchanged.

Process-only HOME, CODEX_HOME, CODEX_SQLITE_HOME and TMPDIR point into this root.
The new config selects file-based model/MCP credential storage and contains no
MCP server definition or login. No personal account/credential is used. The
generated schema confirms this installed client exposes MCP status/tool-call
methods without requiring a model turn; no tool call was made in this pass.

Actual results:

- Copied executable reports Codex CLI0.164.0.
- Initial strict deny-default offline profile aborted both canaries; it was not
  accepted as working isolation.
- Revised offline process policy permits the disposable allowed-file canary
  (exit0) and denies the separate disposable blocked-file canary (exit1).
  It blocks network and selected credential/launch/TCC services; full OS-store
  confinement was not verified and this is not native/MCP isolation acceptance.
- The actual separate Codex app-server initialized and MCP status returned0
  configured servers. Only that owned process was stopped afterward.
- No MCP/provider connection, real installation, model call, native Fetch,
  Keychain read/write or certificate trust change occurred.

Computer use prepared SystemSettings Users&Groups/AddUser; macOS requires the
administrator password at Unlock. No password entered/account created/security
setting changed. Actual protected native session and human confirmation remain
prerequisites. Keep the temporary user-setup prompt for the user's handoff.

Private generated schemas/logs/policies stay outside Git. Future scoped trust
must verify certificates normally; do not use a browser warning bypass or disable TLS.
