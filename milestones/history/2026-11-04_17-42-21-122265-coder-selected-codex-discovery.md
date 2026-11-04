# Read only the selected Codex configuration

- Local timestamp / acting role / milestone: 2026-11-04T17:42:21.122265-05:00 / Coder specialist, recorded by Orchestrator / selected-discovery-scope
- Requested scope: Remove neighboring configurations when selecting .codex.
- Starting repository state: Independent HTTP reproduction fails on original source; focused fixtures updated by Tester. Pre-existing dirty work preserved, including unrelated MCP authentication/restart descriptions.
- Work performed / changed files: src/discovery/mod.rs sends the .codex branch to a singleton config.toml candidate and extracts the existing candidate loop for reuse. src/mcp/management.rs discovery description and src/discovery/info.md, src/api/setup/info.md, src/mcp/info.md match the correction. Root synchronizes API contract, architecture decisions/index/guides, NOTES.md, PROJECT_CONTEXT.md, tests/info.md and milestone records.
- Decisions: No new route, server-specific logic or directory support. Default/home/single-file semantics and existing file errors/read protections remain.
- Checks and evidence: Orchestrator inspected source/test diffs. git diff --check on source and test scope passes. Tester has been told source is ready; final tests/build still pending.
- Resulting state / acceptance: Scoped fix implemented; not yet final acceptance.
- Remaining work and handoff: Focused HTTP discovery checks, native build and independent final evaluation. No native app or live provider/MCP execution.
