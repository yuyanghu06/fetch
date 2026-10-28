# Require stored-credential authenticated verification

- Local timestamp / acting role / milestone: 2026-10-27T09:54:47.790641-04:00 / Orchestrator (serialized by Orchestrator) / mcp-mvp-architecture.
- Requested scope: Account for actual server authentication/storage and determine whether MCP returns provider tokens; incorporate the user's successful-store plus stored-credential authenticated-service 2xx requirement and clarification.
- Starting repository state: Documentation-only project; D20–D23 approved, authentication route policies pending, encrypted server-scoped Fetch-managed secrets selected, implementation paused.
- Work performed / changed files: Updated research/API/storage/decisions/open questions/context/notes/instructions/index/readiness/guides and milestone/to-do. Added D24 integration coverage and D25 successful encrypted storage/readback then authenticated service 2xx gate. Asked and received clarification that lifeboat meant the service authenticated test endpoint.
- Decisions: D24/D25 requirements confirmed. Exact service endpoints, wrapper-owned encrypted-store compatibility, non-token verification, background recheck/refresh and saved-but-unverified cleanup remain open. Prior 10-minute/overlap proposals not approved; GET polling remains read-only.
- Checks and evidence: Inspected Researcher report and independently opened the central MCP authorization/elicitation specification clauses. Source-backed findings and maintained examples recorded; final format/link/JSON/history checks and independent evaluation follow serialization. No application tests.
- Resulting state / acceptance: Auth/storage requirements documented; independent evaluation pending.
- Remaining work and handoff: Verify the update, explain the standards distinction and retained storage constraint, then review still-open integration and route policies with the user.

Append this entry once under `milestones/history/` with a chronological,
collision-safe filename. Corrections belong in a new entry.
