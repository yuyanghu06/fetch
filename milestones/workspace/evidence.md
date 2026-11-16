# Workspace — source, fixtures and signed build

November12,2026. The selected first slice is implemented and independently ACCEPTed.

- Six disposable backend/API checks pass, including actual native-authorized
  HTTP handlers, forbidden ordinary bearer/background access, in-place file save,
  cross-app draft conversion, saved-model metadata privacy and plain/compressed
  UTF-8 session reconstruction without source changes.
- Seven library checks pass: four cache/fake-protocol cases and three Workspace
  widget/editor flows. The editor regression keeps pending model/save operations
  attached to their selected file and rejects old-project replies.
- Existing shared navigation regression passes. Total14 unique focused checks.
- Signed development build-only succeeds in13.28seconds, including strict
  signature verification. Local Rust/Apple tools and selected Apple Development
  identity/private-key availability checked. Existing compiler/deprecation warnings remain.
- git diff --check passes. No running Fetch relaunch, actual client/model/provider
  call, personal file/config/credential/trust change, commit/push or release.

[Exact final results](evidence/verification.txt), [global](evidence/workspace-global-874x630.png),
[project](evidence/workspace-project-874x630.png), [minimum controls](evidence/workspace-global-minimum-760x520.png),
[expanded session](evidence/workspace-session-expanded-874x1000.png),
[converted draft editor](evidence/workspace-pasted-agent-editor-874x1000.png).

The first fixture run exposed a canonical macOS path-alias save bug; fixed and
retested. Independent review exposed an asynchronous editor-target bug; fixed and
covered. A later compiler run ran out of disk space. Removed only59 generated
incremental cache folders older than one day (2038586403bytes); source, binaries,
app data and active work preserved, and final checks/build passed.

## Remaining acceptance and deliberately deferred work

User must explicitly Quit Fetch and reopen the signed development app to see the
new Workspace tab. Actual personal-file editing/clipboard and real Claude catalog
startup remain unobserved; fixture evidence is not a live-account claim. Codex's
saved list is labeled and is not an account entitlement check. Compressed chunks
re-decode the prefix with bounded memory; raw fork references are preserved rather
than reconstructing inherited conversations.

User explicitly chose Workspace-first delivery: no session tools exist on MCP.
Those await the separate reviewed project-bound connection feature. Instructions
and subagent definitions stay accessible only through Fetch's native private APIs.
