# Add scheduled app server refresh

- Local timestamp / acting role / milestone: 2026-11-07T22:48:47.285102-05:00 / Coder, recorded by root / periodic-server-refresh
- Requested scope: Automatic app server-list GET each minute, as explicitly clarified by user.
- Starting repository state: Existing1sactive-work polling and prior healthy-auth header edit; preserve unrelated changes.
- Work performed / changed files: Coder owns src/ui/mod.rs only. Added AutomaticRefresh message, separate60second Iced time subscription, and handler using existing refresh()/polling sequence. Handler skips absent client or in-flight automatic/status read. Root inspected exact diff and updates UI guide/journal.
- Decisions: No lower Runner change, new route/dependency/state field, auth authority change, or native launch. Existing1sTick and manual Refresh remain.
- Checks and evidence: Root inspected source: subscription60s→AutomaticRefresh→Desktop::refresh→NativeClient::servers→GET /api/v1/servers. Existing server reply clears matching polling sequence; close_authentication clears polling on close.
- Resulting state / acceptance: Source implemented; independent focused tests/build underway.
- Remaining work and handoff: Tester owns src/ui/auth_tests.rs; then final independent evaluation. No live GET/app/MCP/account/personal-state test.
