# Integrate restart approval and server-route review

- Local timestamp / acting role / milestone: 2026-10-27T08:50:40.217869-04:00 / Orchestrator / mcp-mvp-architecture
- Requested scope: Record restart approval, reuse the existing GET, move servers outside setup and incorporate the updated collaborative-feedback preference.
- Starting repository state: Documentation-only planning repository. D20 approved; GET additions proposed. Two independent reviews ACCEPT, including D21 relocation.
- Work performed / changed files: Synchronized current instructions/context/notes, architecture decisions/contracts/index/open questions/readiness/storage/research and folder guides; marked handoff body historical; serialized Planner and Evaluator evidence; refreshed milestone/to-do/history guides. No app source, dependencies, services, credentials, personal configs or immutable entries changed.
- Decisions: D20 confirms presented restart behavior. D21 puts existing server operations under /api/v1/servers, keeping the /api/v1 prefix and setup discovery/installation. New ordinary GET fields/rules remain unapproved. URL changes do not choose modules or storage lifetime.
- Checks and evidence: Inspected Planner and both independent Evaluator ACCEPT reports. Prior automated checks passed 43 Markdown files, 11 JSON examples, 190 local links and 20 history entries; final link/anchor/JSON/template/history/format validation runs after integration. No application tests.
- Resulting state / acceptance: Accepted documentation integrated; architecture active and incomplete, implementation paused.
- Remaining work and handoff: Obtain user feedback on the full ordinary response proposal for GET /api/v1/servers, then finish other setup/server-management contracts. Preserve PostgreSQL/retention deferral and local-file/encrypted-token split.

Append this entry once under `milestones/history/` with a chronological,
collision-safe filename. Corrections belong in a new entry.
