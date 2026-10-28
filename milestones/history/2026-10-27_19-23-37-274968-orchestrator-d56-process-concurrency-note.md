# Record configurable per-MCP processes and queue distribution

- Local timestamp / acting role / milestone: 2026-10-27T19:23:37.274968-04:00 / orchestrator / mcp-mvp-architecture
- Requested scope: User asks to note desired total concurrent processes per MCP and queued work distribution, maybe outside MVP.
- Starting repository state: D51–D55 selected; current queued execution/settings proposal has one initial wrapper and unresolved additional policy details; no implementation.
- Work performed / changed files: Added D56 and dated NOTES entry; clarified process versus call concurrency in execution/context/AGENTS/open questions/index/guides and milestone backlog.
- Decisions: Desired capability confirmed; MVP inclusion undecided. No count/algorithm/autoscaling/state isolation or new process launch permission selected. D51 queuing remains active.
- Checks and evidence: Narrow note synchronized and documentation checker run; no new independent review or runtime tests claimed. No source/process/service/credential changes.
- Resulting state / acceptance: Feature preserved in canonical architecture and project notes without silently adding MVP scope or deferring existing queues.
- Remaining work and handoff: Continue remaining current execution review; separately decide process-pool scope and full API/lifecycle/affinity design before building it.
