# Folder discovery and automatic Install page

- Local timestamp / acting role / milestone: 2026-11-03T19:25:58.185229-05:00 / Coder and Orchestrator UI implementation / automatic-config-discovery.
- Requested scope: discover known configuration files automatically, with optional explicit file/folder selection.
- Starting repository state: prior clean source; read-only Planner findings recorded. Test work concurrent with exclusive ownership.
- Work performed / changed files: Coder changed src/discovery/mod.rs, src/api/setup/mod.rs, src/mcp/management.rs; Orchestrator changed src/ui/mod.rs, src/ui/file_picker.rs, src/native_auth/bridge.m and affected architecture/context/testing/folder documentation.
- Decisions: existing config_path now supports file, home or .codex directory; no route, request-body or installation selection change. UI discovers first visit and keeps custom controls optional. Known paths only, no recursive reads.
- Checks and evidence: inspected complete code diffs; git diff --check passes. Existing management invocation already calls the same scan helper. Independent tests underway; no app or personal config read.
- Resulting state / acceptance: implementation ready for independent fixture checks and evaluation; no runtime UI claim.
- Remaining work and handoff: fixture and build results, independent evaluation/fixes and final integration journal.
