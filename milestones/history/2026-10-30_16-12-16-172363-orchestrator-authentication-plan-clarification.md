# Record the authentication-route correction and implementation plan

- Local timestamp / acting role / milestone: 2026-10-30T16:12:16.172363-04:00 / Orchestrator / local-working-mvp.
- Requested scope: Summarize how the existing authenticated-check contract and parallel-service behavior will be implemented; user emphasizes POST authenticate.
- Starting repository state: main fe916e7; working tree clean before this documentation pass; live remote main matched at the earlier status check.
- Work performed / changed files: Inspected the Planner report and relevant source/contracts. Updated API clarification, NOTES.md, milestone snapshot/to-do, local milestone guide, architecture folder guide and these immutable history records. No application source changed.
- Decisions: Correct the earlier explanation that blended POST authentication with PATCH credential saves. Keep D25/D31. Plan per-server UI requests and supported credential-group checks. Parallel services are independent of the number of values one service requires. No new route or weakened success rule selected.
- Checks and evidence: POST uses stored readback and verify_stored before authenticated; PATCH currently checks exactly one set entry with no removals; backend locks use server IDs; UI uses one outstanding auth_request/global busy. Documentation diff inspected. No new tests/builds/live provider/MCP/native actions.
- Resulting state / acceptance: Bounded source-backed plan recorded, not implemented or independently runtime-accepted. Prior build/fixture evidence remains scoped to prior source.
- Remaining work and handoff: Implement/review/test targeted fixes under existing authorization when continuing coding. Deletion-only/non-auth success behavior and protected-MCP authentication proof remain unresolved; retain native isolation/human confirmation and no release boundaries.
