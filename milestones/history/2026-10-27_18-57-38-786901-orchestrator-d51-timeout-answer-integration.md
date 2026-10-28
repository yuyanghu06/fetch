# Integrate queue correction and timeout discovery answer

- Local timestamp / acting role / milestone: 2026-10-27T18:57:38.786901-04:00 / orchestrator / mcp-mvp-architecture
- Requested scope: Carry queue/concurrency requirement forward while answering whether per-tool timeouts can be read.
- Starting repository state: Research complete and independent Evaluator ACCEPT; revised scheduler planning still underway.
- Work performed / changed files: Inspected source/reports and current docs, integrated D51 queue requirement and withdrawn fixed limit with no arbitrary fallback adoption; updated tracking/guide evidence.
- Decisions: Known documented limits may inform a proposed policy; arbitrary timeout args unchanged. Queue wait separate from dispatched work; no universal metadata discovery assumed.
- Checks and evidence: Docs checker passed; independent evaluation ACCEPT. No source, dependencies, services, VM, credentials or live MCP tests.
- Resulting state / acceptance: Question answered with verified protocol boundary, queue requirement saved; architecture remains active/incomplete.
- Remaining work and handoff: Inspect remaining Planner report before complete revised queue/concurrency/cancellation/restart/aggregate-state/fallback contract; obtain user input on new parameters.
