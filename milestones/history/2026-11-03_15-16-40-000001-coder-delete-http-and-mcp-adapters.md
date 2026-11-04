# Implement deletion HTTP and MCP adapters

- Local timestamp / acting role / milestone: 2026-11-03_15-16-40-000001 America/New_York / Adapter Coder, inspected and serialized by Orchestrator / delete-server.
- Requested scope: Wire one DELETE operation using current API conventions and shared backend/permission rules.
- Starting repository state: Approved deletion plan, unrelated concurrent docs preserved; backend writer owns lifecycle/storage separately.
- Work performed / changed files: Adapter owns api/mod.rs, api/runtime.rs, mcp/management.rs and new api/v1/servers/server_id/delete.rs/module plumbing. Root inspected diff and added folder guides. Both HTTP and MCP invoke Manager::delete_server(id,caller).
- Decisions: Use URL-shaped new handler rather than runtime.rs accumulation. Reuse scope/target/error helpers with parent visibility. Reject query/nonempty body; no force or UI redesign. Add descriptive fetch__delete_server metadata and permission mapping.
- Checks and evidence: Source reads confirm route wiring, caller checks before input/state, hidden404, common503/error conversion and exact management argument allowlist. No builds/tests/checks/format/lint/runtime/apps/providers/MCP/personal-state operations ran.
- Resulting state / acceptance: Adapter source frozen; backend still being implemented. No compilation/runtime acceptance claimed.
- Remaining work and handoff: Integrate backend, independently verify/evaluate actual source, document/deliver deletion before queued shared authentication work.
