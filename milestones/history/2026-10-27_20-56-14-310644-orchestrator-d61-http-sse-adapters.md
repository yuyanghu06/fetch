# Record routed HTTP and SSE transport functions

- Local timestamp / acting role / milestone: 2026-10-27T20:56:14.310644-04:00 / orchestrator / mcp-mvp-architecture
- Requested scope: Record user direction to add HTTP/SSE as separate functions selected by the shared runner.
- Starting repository state: D59 shared runner/D60 streaming selected; remote scope previously proposed; implementation paused.
- Work performed / changed files: Added D61; updated remote guide, current API/discovery boundaries, context/notes/AGENTS, index/open questions/readiness/audit and milestone tracking/guides.
- Decisions: HTTP/SSE upstream support selected alongside local-process adapters, replacing D35 local-only scope. Exact API/lifecycle/version/annotation/recovery mechanisms remain open; no blanket prior proposal approval.
- Checks and evidence: Root inspected actual records against user statement; reused completed primary-source research, no new unfamiliar implementation or live tests. Documentation checker follows.
- Resulting state / acceptance: Accepted transport/function organization recorded; no source code, service/config/credential changes.
- Remaining work and handoff: Independently evaluate synchronization, then continue exact remaining amendments without reopening adapter organization.
