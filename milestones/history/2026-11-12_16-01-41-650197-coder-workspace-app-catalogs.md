# Workspace app catalogs

- Local timestamp / acting role / milestone: 2026-11-12T16:01:41.650197-05:00 / coder / Workspace.
- Requested scope: Build Workspace with frontend API wrappers, native-only file/config operations and read-only chunked sessions.
- Starting repository state: Existing unrelated connector, server on/off, UI and documentation work preserved. New Workspace core/UI under construction; no project-bound MCP feature exists.
- Work performed / changed files: Implemented src/workspace/models.rs/model_tests.rs: bounded saved Codex cache excluding private metadata, no Codex startup; Claude initialization-only strict empty MCP/tools, timeout/output limits and owned-process cleanup. Executable lookup includes local/Homebrew locations. Tests use fake processes/disposable cache only.
- Decisions: User explicitly approves native Workspace routes and selects finishing Workspace first, keeping all session MCP unavailable. Original files remain authoritative; no log archive or writes. Codex models are labeled saved app catalog.
- Checks and evidence: Root's focused API/core suite6 passes, catalog/UI suite7 passes before final style retest, existing navigation regression1 passes. Four/five offscreen widget captures inspected as recorded. Initial concrete path/editor findings fixed. A later compiler run ran out of disk; removed only59 old generated incremental caches (2038586403 bytes), preserving source/executables/app data. No real app/client/model/MCP execution or personal-state edits.
- Resulting state / acceptance: Scoped implementation and independent source review accepted pending final checks/build. API-first frontend is implemented; session MCP deliberately absent.
- Remaining work and handoff: Final style/widget retest, signed development build-only, final evidence and journal integration. Actual native personal-file/clipboard and Claude catalog acceptance remain unobserved; project MCP is a separate selected follow-up.
