# Config picker integration

- Local timestamp / acting role / milestone: 2026-10-28_17-31-23-020389 America/New_York / Orchestrator / config-file-picker.
- Scope: implement native configuration-file chooser without changing API/import behavior.
- Starting state: prior local development-default changes preserved.
- Work/files: inspected src/ui/mod.rs, src/ui/file_picker.rs and src/native_auth/bridge.m; integrated independent source-verification/acceptance. Updated src/ui/info.md, src/native_auth/info.md, docs/testing/DEVELOPMENT.md, PROJECT_CONTEXT, NOTES and milestone journal.
- Evidence: cargo build --locked --bin fetch --message-format short exited0; Finished dev profile in11.35seconds. Existing four Keychain deprecation warnings plus one unread protocol-field warning remain. No executable, native dialog, test or MCP connection was run by agents.
- Behavior: Choose file… opens standard async single-file sheet, hidden folders visible; selection fills absolute path, Cancel preserves prior value. No file-content read/discovery/install from choosing. Main-queue/callback lifetime/duplicate/stale-window guards accepted in independent source reviews.
- Result: source integrated locally and native binary compiles; no runtime UI acceptance claimed. No new dependencies/build linkage changes; no unrelated file changes discarded, no personal settings/credentials/trust change, no commit/push or release.
- Remaining: manually exercise picker success/cancel/hidden files/repeated click/close-reopen/Quit after relaunch. Preserve earlier pending client/provider acceptance and four unrun environment-path tests.
