# Research MCP and provider-token ownership

- Local timestamp / acting role / milestone: 2026-10-27T09:54:47.789921-04:00 / Researcher (serialized by Orchestrator) / mcp-mvp-architecture.
- Requested scope: Account for actual server authentication/storage and determine whether MCP returns provider tokens; incorporate the user's successful-store plus stored-credential authenticated-service 2xx requirement and clarification.
- Starting repository state: Documentation-only project; D20–D23 approved, authentication route policies pending, encrypted server-scoped Fetch-managed secrets selected, implementation paused.
- Work performed / changed files: Read-only primary-source research of MCP 2026-07-28 authorization/URL elicitation, OAuth token response and maintained GitHub MCP v2.0.3 / TypeScript SDK v2.4.0. Checked releases, licenses, dependencies, documented lifecycle limits and refresh issue context; no specialist writes.
- Decisions: No universal local provider-token export exists. HTTP client OAuth and wrapper-owned third-party OAuth are distinct. Recommendations remain proposals; no protocol capability, wrapper-store exception, library or mechanism selected.
- Checks and evidence: Direct normative URLs and pinned source links recorded in research.md. GitHub local OAuth tokens documented in memory only; SDK storage hooks are client application interfaces. Current SDK license transition differs from historical MIT-only assumptions. No Fetch login/runtime tests.
- Resulting state / acceptance: Research inspected and integrated into the explicit integration contract; no implementation.
- Remaining work and handoff: Define token owner/store/handoff/refresh and compatible test evidence before automatic support; do not bypass URL-mode provider-token boundary.

Append this entry once under `milestones/history/` with a chronological,
collision-safe filename. Corrections belong in a new entry.
