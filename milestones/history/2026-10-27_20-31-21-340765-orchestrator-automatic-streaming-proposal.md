# Explain automatic MCP streaming default

- Local timestamp / acting role / milestone: 2026-10-27T20:31:21.340765-04:00 / orchestrator / mcp-mvp-architecture
- Requested scope: Answer whether Fetch can stream by default and needs a user-facing streaming mode.
- Starting repository state: D59 shared handlers selected; D47 dual-revision transport selected; exact streaming default unchosen; app implementation paused.
- Work performed / changed files: Rechecked both primary transport specifications and documented a complete proposed automatic streaming policy with open question, note and milestone guides.
- Decisions: Recommend SSE for tool calls/selected ongoing operations and JSON for quick reads without a toggle; retain native acknowledgements/errors, raw terminal results, capability scope and version cancellation/deadlines. Proposal only.
- Checks and evidence: Primary transport specs allow server-selected JSON/SSE and require both client accept types; inspected actual doc changes. Documentation checker follows; no runtime test.
- Resulting state / acceptance: Question answered by a source-backed recommendation; no new approval inferred or application code created.
- Remaining work and handoff: Obtain feedback on proposed default and preserve remaining capability/transport scope decisions.
