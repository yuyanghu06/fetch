# Accept existing multiple-instance import behavior

- Local timestamp / acting role / milestone: 2026-11-03T17:09:36.297254-05:00 / Orchestrator / shared-mcp-authentication.
- Requested scope: User accepts current distinct-entry imports and says the additional instance-creation control is not needed for now.
- Starting repository state: Shared router delivered; current ID/import behavior explained; pre-existing documentation edits preserved; Google integration pending.
- Work performed / changed files: Recorded scope in NOTES.md, PROJECT_CONTEXT.md, authentication-engine.md, snapshot, to-do and milestone guide. No application changes.
- Decisions: Existing separate imported entries suffice; do not add duplicate-instance UI/API/install behavior. Preserve independent per-instance credentials/accounts and current same-source reuse. Google remains pending.
- Checks and evidence: Inspected updated documentation. No tests/build/helper/provider/personal-state action; prior99-fixture/native-build evidence is unchanged.
- Resulting state / acceptance: Extra duplicate-instance control removed from active work by explicit user direction.
- Remaining work and handoff: Continue requested behavior discussion; existing Google deferral and broader MVP limits remain.
