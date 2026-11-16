# server first tools plan

- Local timestamp / acting role / milestone: 2026-11-12T15:47:07.743022-05:00 / planner / server-first-tools
- Requested scope: Implement selected discovery/call flow, startup IDs and selected-server optimizations.
- Starting repository state: Prior isolated connector acceptance with observed actual Claude tools-fetch failure. Substantial unrelated Workspace/server-Off/UI changes remain.
- Work performed / changed files: Read-only Planner bounded fixed catalog, safe repository inventory, explicit MCP server filters, direct raw generic dispatch, continuation field retention, parallel collection, transactional aliases and focused acceptance. Ownership split into nonoverlapping source hunks; tests owned independently. Architecture contract and journals updated by root.
- Decisions: User accepts generic caller after approval-screen explanation. No per-connection expansion/new HTTP route/auth changes.
- Checks and evidence: Inspected source, primary sources and specialist reports; no new runtime/clients/config/credential/trust operations.
- Resulting state / acceptance: Plan accepted; implementation underway, tests/review/build pending.
- Remaining work and handoff: Integrate focused coders, independent fixture tests/review; manual actual-client acceptance remains separate.
