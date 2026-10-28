# Review the server-route relocation

- Local timestamp / acting role / milestone: 2026-10-27T08:50:40.216304-04:00 / Evaluator / mcp-mvp-architecture
- Requested scope: Read-only focused review of D21 moving existing server operations out of setup.
- Starting repository state: D20 approved, ordinary GET proposal independently accepted as documentation; user subsequently requested route relocation.
- Work performed / changed files: Evaluator inspected canonical paths, approved query/restart behavior, setup exceptions, module status, approval boundaries and feedback preference. No specialist writes; report serialized by Orchestrator.
- Decisions: ACCEPT; no material issues. /api/v1/servers family is current, discovery/installation retain /api/v1/setup. GET additions are still proposed; implementation remains paused.
- Checks and evidence: Current contracts consistently use new paths; old canonical URLs only appear in supersession statements, historical records have a current notice. ID/attempt filters and D20 behavior preserved; no new source tree or aliases authorized. This review was documentation only.
- Resulting state / acceptance: D21 documentation accepted; no application tests.
- Remaining work and handoff: Present the relocated existing route and obtain feedback on pending GET fields/rules; continue the seven-step order.

Append this entry once under `milestones/history/` with a chronological,
collision-safe filename. Corrections belong in a new entry.
