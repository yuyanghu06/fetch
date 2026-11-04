# Limit discovery to the selected Codex folder

- Local timestamp / acting role / milestone: 2026-11-04T17:40:29.854519-05:00 / Planner (independent read-only specialist), recorded by Orchestrator / selected-discovery-scope
- Requested scope: Selecting `.codex` must read only its config.toml, not neighboring Claude/OpenCode configurations.
- Starting repository state: Large pre-existing dirty tree; src/discovery/mod.rs and tests/backend.rs have no prior diff. Shared AGENT_WORKFLOW.md and ~/.codex/agents role files are absent; specialist follows the session role definition.
- Work performed / changed files: Read discovery branch, backend fixtures, MCP description and API contract. No application files changed in this pass.
- Decisions: Reuse existing bounded candidate reads. Preserve automatic scans, explicit home scans, explicit single-file reads, installed associations and existing failure handling. Update current descriptions to supersede the old parent expansion.
- Checks and evidence: Source inspection locates expansion in src/discovery/mod.rs. Existing HTTP fixture creates neighboring Claude/OpenCode configs and expects all three for .codex; adjust that expectation, home partial-failure input and installed-group indexing. No tests run yet.
- Resulting state / acceptance: Plan accepted for the user's explicit correction. Source/test implementation pending.
- Remaining work and handoff: Implement bounded read change; run focused HTTP discovery tests and native build; independent final evaluation. No app launch, real MCP execution, credentials or personal settings changes.
