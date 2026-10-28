# Defer PostgreSQL and retention while preserving local state requirements

- Local timestamp / acting role / milestone: 2026-10-26T23:49:45.222140-04:00 (America/New_York) / orchestrator, serialized by Orchestrator / mcp-mvp-architecture.
- Requested scope: User asks to sideline retention because initialMVP should have no PostgreSQL functionality and asks what other parts depend on databases.
- Starting repository state: Documentation-only project with earlier PostgreSQL/Tailscale/migration assumptions and an unanswered24-hour restart-retention proposal; prior handoff complete but user continued scope discussion.
- Work performed / changed files: Created storage-scope.md and D18; synchronized AGENTS/context/notes,API/restart/index/openquestions/checklist/roadmap,handoff,snapshot/to-do and affected guides. Serialized read-only Planner audit and independent evaluation. Preserved historical decision/pass records and unfinished work.
- Decisions: Confirmed latest user scope only: no PostgreSQL initialMVP, retention sidelined. PostgreSQL provisioning/connections/database-host Tailscale/schema/migrations and durable retained-result assumptions deferred. Core MCP operations need no database server inherently; saved server metadata, encrypted credentials/auth and optional shared prompt/history still need data storage. No JSON/SQLite/Keychain/RAM-only selection, memory cancellation or approval of other restart policies.
- Checks and evidence: Orchestrator check passed34 Markdown files/9 JSON examples with links/anchors/fences/headings/whitespace. Independent Evaluator passed35 files/9 examples after stale wording corrections. Actual audit found no application/manifests/SQL/runtimeDBdependency. No code, DBuninstall, credentials, personal configs, packages, services, commits or deployment changed.
- Resulting state / acceptance: Storage amendment and dependency audit saved, handoff current. Architecture milestone incomplete and implementation paused. Current-operation async IDs/progress remain accepted.
- Remaining work and handoff: Explain which features need saved/protected local state without claiming PostgreSQL is inherent. Review the local-state design with the user in the selected sequence; continue complete contract reviews when directed. Future PostgreSQL work remains deferred, not implemented.

Append this entry once under `milestones/history/` with a chronological,
collision-safe filename. Corrections belong in a new entry.
