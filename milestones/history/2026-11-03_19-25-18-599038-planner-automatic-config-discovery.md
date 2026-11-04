# Automatic config discovery plan

- Local timestamp / acting role / milestone: 2026-11-03T19:25:18.599038-05:00 / independent Planner / automatic-config-discovery.
- Requested scope: find configuration files automatically, accepting a base folder without requiring manual file location.
- Starting repository state: clean tree; default no-path discovery exists, Install does not call it automatically, explicit paths require files.
- Work performed / changed files: read-only source plan covering discovery service, API, MCP tool, UI and fixture boundaries; Orchestrator records this report.
- Decisions: keep selected file/name installation submission; extend optional config_path to file or known base folder; automatic discovery when first opening Install; explicit override controls.
- Checks and evidence: Planner traced src/discovery/mod.rs, src/api/setup/mod.rs, src/api/runtime.rs, src/mcp/management.rs and src/ui/mod.rs. No tests ran in the planning pass.
- Resulting state / acceptance: bounded plan accepted for implementation; this is no runtime acceptance.
- Remaining work and handoff: implement, independently test, evaluate and document actual outcomes.
