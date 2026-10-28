# Propose a concrete HTTP and MCP wrapper notice location

- Local timestamp / acting role / milestone: 2026-10-27T14:24:44.247655-04:00 / orchestrator / mcp-mvp-architecture
- Requested scope: Answer where wrapper notice belongs in Fetch response.
- Starting repository state: D40 model-visible disclosure selected; exact placement/channel/fields unselected.
- Work performed / changed files: Added illustrative top-levelHTTPresponse_context beside rawresult and separateMCPtext-rendering candidate; updatedopenquestions/notes/guide/milestone/to-do.
- Decisions: HTTPenvelope hasnoouterMCPcontentarray. Proposednoticefield outside result; fullMCPpayload/schema stillunselected and rawdataretained.
- Checks and evidence: Existing primary-source research/Planner guidance reused; docchecks run on modified files. No live/modeldelivery tests.
- Resulting state / acceptance: Concrete placement ready for review as a proposal, not accepted mechanism or implementation.
- Remaining work and handoff: Independently check narrow clarification then answer with smallHTTPexample; MCPmapping remains pending.
