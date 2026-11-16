# Remove the ready runtime hover text

- Local timestamp / acting role / milestone: 2026-11-09T17:39:09.327774-05:00 / Orchestrator, sequential implementation/check/review / server-header-auth-status.
- Requested scope: remove the user's pictured Runtime: ready hover state in the native app.
- Starting repository state: prior navigation fix plus unrelated Figma notes/history uncommitted; preserved.
- Work performed / changed files: src/ui/mod.rs renders ready server identities directly rather than wrapping them in the runtime tooltip. Other runtime statuses retain the existing tooltip; colored dot, disclosure, header actions and backend data remain unchanged. Updated source guide, UI state index, project context/notes and milestone records.
- Decisions: a small conditional display change needs no new test infrastructure or speculative cases. Work performed directly at user's request; review is not independent.
- Checks and evidence: existing actual-widget server-panel/disclosure/action/key-ownership test PASS (1 test). ./scripts/run-development.sh --build-only PASS, 9.09s build plus signed executable and launcher strict verification. Changed-source whitespace check PASS. Direct source review ACCEPT: ready branch contains no tooltip, all other header behavior retained.
- Resulting state / acceptance: requested change implemented and signed development executable updated; no native app launch or runtime hover observation claimed.
- Remaining work and handoff: user explicitly Quits Fetch and reopens through ./scripts/run-development.sh. Prior navigation/Figma/runtime follow-ups remain. No account, credential, MCP, client settings, commit/push or release changes.
