# Separate the disclosure arrow from the name code box

- Local timestamp / acting role / milestone: 2026-11-16T13:38:02.656240-05:00 / Coder, integrated by Orchestrator / workspace-session-error
- Requested scope: retain the visible expand/collapse state cue with requested code-box tool names.
- Starting repository state:12follow-up tests pass; initial18px sizing-only correction still failed the new visible-chevron regression. Its signed build passed30.37s, but was not accepted visually.
- Work performed / changed files: components.rs code_disclosure places its existing18px arrow outside the inset name box; whole header remains one transparent clickable disclosure. No global surface/asset change.
- Decisions: code-box wraps only name, matching user request; arrow stays separate.
- Checks and evidence: diff check passes; first failed targeted log/capture retained in /tmp/fetch-tool-path-followup-tester.sZcnyy/chevron-final. Updated targeted check/captures running in chevron-fixed.
- Resulting state / acceptance: source stable for final visual check.
- Remaining work and handoff: accept only after visible collapsed/expanded arrow, targeted test and final build/review. No personal files/apps/services.
