# Approve common config scope and propose credentials contract

- Local timestamp / acting role / milestone: 2026-10-27T11:06:52.706717-04:00 / orchestrator / mcp-mvp-architecture
- Requested scope: User defers scattered native settings standardization beyond MVP, keeps current standardized fields and says everything else looks good, move on.
- Starting repository state: D29 expanded individual settings scope; three-field draft pending; prior auth/restart/GET approvals unchanged; implementation paused and no runtime code present.
- Work performed / changed files: Recorded D30 approval of reviewed common args/working_directory/environment-ref contract and deferral of D29 native settings normalization. Renamed approved anchor/cross-links, refreshed context/notes/checkpoints/guides/to-do and drafted full manual credentials proposal. Immutable history preserved; root owns docs.
- Decisions: Common fields are Fetch-shared, not a universal MCP schema. Credential proposal has Settings-only set/remove, encrypted save/readback, no secret read API, conservative auth invalidation, no provider/process effects and new busy rejection while login/restart active. New proposal is not approved.
- Checks and evidence: Self-check passed 68 Markdown files,16 JSON examples,248 local links,47 anchors,44 prior linked history entries before two new records; fences/whitespace/template/history checks passed. No application/credential/runtime tests.
- Resulting state / acceptance: D30 and complete next-route proposal synchronized; independent documentation review pending.
- Remaining work and handoff: Obtain independent evaluation, fix material findings and present complete credentials contract for user input. Keep implementation paused and native-settings normalization deferred.
