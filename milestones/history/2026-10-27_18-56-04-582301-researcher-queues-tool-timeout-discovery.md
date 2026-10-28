# Research bounded call queues and per-tool timeout discovery

- Local timestamp / acting role / milestone: 2026-10-27T18:56:04.582301-04:00 / researcher / mcp-mvp-architecture
- Requested scope: User requires queues/concurrency, then asks whether individual tool timeouts can be discovered instead of fixed120.
- Starting repository state: D50 call policy selected; prior execution cap1/noqueue/fixed120 all unapproved proposals.
- Work performed / changed files: Dedicated Researcher checked MCP schemas/timeout guidance, Tokio bounded concurrency/queue cancellation semantics and maintained gateway patterns; root also checked current/legacy Tool schema.
- Decisions: No standardized Tool timeout; input/custom fields not universal deadlines. Queue count+bytes, FIFOstart, atomic dispatch/cancel and separatequeue/run timing needed; exact policy not chosen.
- Checks and evidence: Primary-source and gateway reports inspected before revised Planner; no files changed by specialist or runtime tests.
- Resulting state / acceptance: Research complete; reliable timeout answer available and revised execution plan underway.
- Remaining work and handoff: Answer direct question, retain queue requirement and review exact configurable fallback/limits/settings later.
