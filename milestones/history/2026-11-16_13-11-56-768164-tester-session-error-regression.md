# Session error regression

- Local timestamp / acting role / milestone: 2026-11-16T13:11:56.768164-05:00 / tester / workspace-session-error
- Requested scope: real editor/tab behavior with late missing-rollout failure.
- Starting repository state: session error separation implemented.
- Work performed / changed files: Tester extends src/ui/workspace_tests.rs using disposable UI/API replies; no personal-state actions.
- Decisions: use 874x900 fixture so Save is visible; initial 750px fixture clipped it.
- Checks and evidence: exact focused test session_list_failure_stays_in_sessions_while_codex_instructions_remain_editable passes (1.36s). Original failed fixture log and retry/captures preserved at /tmp/fetch-workspace-session-error-tester.rPuGPx.
- Resulting state / acceptance: instructions stay editable; exact error only in Sessions; successful refresh clears it and preserves unsaved content.
- Remaining work and handoff: final Workspace regression after requested rendering/color changes, reader regression and final build/review.
