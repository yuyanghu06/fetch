# Add the server-header delete button

- Local timestamp / acting role / milestone: 2026-11-03_16-20-25-000001 America/New_York / Coder, independent Tester and Orchestrator / server-delete-control.
- Requested scope: Small × beside each collapsed/expanded server heading invoking existing DELETE, prioritized for user testing before shared authentication rewrite.
- Starting repository state: Delete route64cff43 pushed; unrelated concurrent documentation preserved. Auth research complete, no auth implementation yet.
- Work performed / changed files: Five UI files: shared header/theme, DELETE request/pending/cache cleanup, read invalidation and one required test fixture initializer. Root inspected diff and updated guides/coverage/journal. No new tests for the small control.
- Decisions: Direct DELETE with encoded ID/no body; disable known busy/pending states; retain other rows' responsiveness; remove only on confirmed response, refresh uncertain outcomes and never auto replay. Preserve stale response/window ownership and existing authentication requests.
- Checks and evidence: Independent source ACCEPT; CLT-scoped cargo build --locked --bin fetch --message-format short exit0/19.57s. CLT-scoped cargo test --locked --lib ui::auth_tests -- --nocapture exit0,14passed/0failed/68filtered,12.51scompile and0.01sexecution. Five native Security deprecations plus unread protocol field warnings; no fixes needed. Existing tests cover auth/message handling, not actual delete clicks.
- Resulting state / acceptance: Native binary ready for user testing. No app/window/Keychain/provider/MCP/personal-client execution or actual deletion by agents. Source/control acceptance is separate from runtime appearance.
- Remaining work and handoff: Commit/push this bounded UI change immediately, then resume shared generic web-auth implementation. No Google-specific route or login engine; provider facts only in narrowly required compatibility descriptions. Changed Figma specimen remains unmapped.
