# Record automatic installation timeout and per-call execution deadline

- Local timestamp / acting role / milestone: 2026-10-27T19:21:55.174527-04:00 / orchestrator / mcp-mvp-architecture
- Requested scope: User clarified configurable timeout is injected as execution deadline and can be set automatically by MCP installation.
- Starting repository state: D51–D54 selected; queued policy independently reviewed as documentation, other proposed clauses pending.
- Work performed / changed files: Recorded D55 and synchronized execution/API/open questions, context/AGENTS/notes, guides/tracking; preserved prior history.
- Decisions: Installer initializes duration from supported explicit server-default source or120fallback and preserves existing user setting; Fetch computes dispatch-relative deadline, separatequeuewait/rawarguments unchanged. No universal metadata or hard-stop guarantee.
- Checks and evidence: Narrow clarification checked against D34/D39/D50/D52–D54 and prior independent source/proposal evidence; docs checker run. No new independent/runtime tests claimed.
- Resulting state / acceptance: Clarification recorded; remaining policy/schema/default choices and implementation still open.
- Remaining work and handoff: Confirm install duration-to-dispatch deadline plainly and continue remaining architecture review without treating clarification as blanket approval.
