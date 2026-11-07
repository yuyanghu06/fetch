# Record405tools and proposed server-first discovery

- Local timestamp / acting role / milestone: 2026-11-06T22:01:33.747121-05:00 / Primary agent directly, note/research discussion / future discovery proposal.
- Requested scope: Note user-observed Fetch405tools/working connection, concern about context growth, and idea of selecting MCP servers before exposing their tools; give candid assessment.
- Starting repository state: Scoped connection fix verified via actual Codex client; current screenshot confirms user's TUI now connected405tools.
- Work performed / changed files: Recorded dated user note in NOTES.md and concise PROJECT_CONTEXT.md. Inspected current combined native catalog and existing server-filtered list helper. Fetched official Codex MCP and OpenAI tool-search docs; documented proposal/tradeoffs/evidence limits in canonical research/open questions. Updated tracking and this history.
- Decisions: No new architecture selected. Recommend investigate actual Codex context behavior and server-first discovery, rather than equate available-tool count with per-request context size. Existing raw metadata/auth/permission/selected protocol requirements remain.
- Checks and evidence: Screenshot confirms connected405tools. Source currently combines all permitted definitions; filtered helper does not narrow initial native catalog. Official API docs support deferred discovery/grouping, but do not prove installed CLI behavior. No runtime/token measurement, tests/builds, source edits, new model turn, subagent or global setting changes.
- Resulting state / acceptance: Requested note saved; proposal remains unselected and actual context cost unknown. Earlier user-refresh follow-up complete from screenshot.
- Remaining work and handoff: If user selects investigation, measure actual client definition/token behavior, compare a small server-first discovery flow, and review exact new/changed API/catalog/session/permissions before implementation.
