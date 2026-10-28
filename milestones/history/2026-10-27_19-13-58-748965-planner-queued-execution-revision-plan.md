# Plan revised queues and configurable deadline policy

- Local timestamp / acting role / milestone: 2026-10-27T19:13:58.748965-04:00 / planner / mcp-mvp-architecture
- Requested scope: User requires queues/concurrency and asks individual timeout discovery instead of universal120.
- Starting repository state: D51 selected; fixed120 draft withdrawn before later D53 fallback clarification; no new queue schema approved.
- Work performed / changed files: Read-only Planner recommended bounded FIFO/count+bytes, separate deadlines, authority/generation recheck, settings API and aggregate state; root inspected report on current turn.
- Decisions: No universal timeout metadata, no input mutation, no implicit unlimited/fallback adoption at that pass. Exact policy defaults remain proposals.
- Checks and evidence: Existing primary queue/timeout research inspected; no edits or runtime tests by Planner.
- Resulting state / acceptance: Bounded revised plan accepted for documentation drafting, not user approval.
- Remaining work and handoff: Incorporate new D52/D53 requirements and present full revised execution/settings contract.
