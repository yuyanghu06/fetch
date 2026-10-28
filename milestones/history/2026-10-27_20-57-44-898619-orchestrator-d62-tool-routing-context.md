# Record per-tool transport configuration and call context

- Local timestamp / acting role / milestone: 2026-10-27T20:57:44.898619-04:00 / orchestrator / mcp-mvp-architecture
- Requested scope: Record user direction that each tool config carries MCP type to select the invocation function.
- Starting repository state: D61 HTTP/SSE adapters selected; raw argument/Tool JSON contracts and implementation pause remain.
- Work performed / changed files: Added D62 and per-tool routing section; aligned API/execution/context/notes/instructions/open questions/index and milestone records/guides.
- Decisions: Carry Fetch-owned connection/server routing alongside calls in internal execution context, outside raw arguments and upstream definitions; exact storage/field representation remains open.
- Checks and evidence: Inspected user wording and existing raw-forwarding policies; documentation checker follows. No runtime code, service calls or config/credential changes.
- Resulting state / acceptance: Routing requirement recorded without silently selecting duplicated storage/reference implementation or public request fields.
- Remaining work and handoff: Obtain independent documentary evaluation; continue remaining exact API/lifecycle/compatibility amendments.
