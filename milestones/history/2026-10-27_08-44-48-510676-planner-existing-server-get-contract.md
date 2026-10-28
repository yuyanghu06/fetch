# Plan the existing server-information GET contract

- Local timestamp / acting role / milestone: 2026-10-27T08:44:48.510676-04:00 / Planner (report serialized by Orchestrator) / mcp-mvp-architecture.
- Requested scope: Bound the next complete contract for existing GET /api/v1/setup/servers after the user's restart approval.
- Starting repository state: Documentation-only project; restart fields previously approved, current user approves remaining bundle. Ordinary GET record/validation/error details unresolved; implementation paused.
- Work performed / changed files: Planner read current instructions/contracts/storage records read-only; proposed safe saved config/credential presence, separate runtime/auth state, restart_required, active ordinary restart, ID ordering/deduplication, ordinary errors and explicit per-server guidance. No specialist writes. Orchestrator inspects this report and serializes journal records.
- Decisions: Planner recommendations remain proposals. User separately reaffirmed reusing the existing GET. Orchestrator omitted optional provenance/auth-method inventories to bound this response, retained the approved minimal filtered restart view and left writes/auth mechanisms for later contracts.
- Checks and evidence: Read-only plan inspected; acceptance scenarios cover all/selected/missing IDs, unknown auth, secret-free reads without original source files or mutations, denied/unreadable records and edits during restart. No application tests run.
- Resulting state / acceptance: Bounded plan used for the ordinary GET section in docs/architecture/api-contract.md; new fields/rules await the user. Restart approval is D20, not blanket acceptance of GET additions.
- Remaining work and handoff: Validate and independently evaluate the documentation, then present the one existing-route proposal for user feedback. Preserve seven-step order, storage deferrals and implementation pause.

Append this entry once under `milestones/history/` with a chronological,
collision-safe filename. Corrections belong in a new entry.
