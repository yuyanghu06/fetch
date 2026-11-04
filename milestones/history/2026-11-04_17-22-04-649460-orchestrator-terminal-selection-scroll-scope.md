# Scope terminal selection and scrolling fixes

- Local timestamp / acting role / milestone: 2026-11-04T17:22:04.649460-05:00 / Orchestrator / terminal-selection-scroll.
- Requested scope: screenshot and the cursor; user clarified large gray blocks and inability to scroll up in the terminal.
- Starting repository state: normal PATH fix accepted, embedded rendering has earlier offscreen acceptance. Existing uncommitted work/concurrent identity work preserved.
- Work performed / changed files: read current vendored mouse/selection/background/wheel code and local official Iced/Alacritty sources. Asked/received targeted clarification. Assigned Planner, Coder(view.rs only) and Tester(UI test file only); milestone/docs only by root.
- Decisions: gray blocks match selection, not a reason to change block cursor shape. Fix proven release-outside stuck-drag bug; investigate pixel-wheel sign and fractional gesture capture. No deliberate selection removal or custom terminal implementation.
- Checks and evidence: source proves outside releases ignored; screenshot alone did not prove cause. Pixel-wheel source subtracts y while Lines uses positive y; local Iced parent consumes unhandled wheel gestures. Independent reproduction/plan next; no tests/build passed for this fix yet.
- Resulting state / acceptance: bounded authorized bug work active, not complete.
- Remaining work and handoff: reproduce → narrow source fix → focused rendered/event checks → native build → independent review. No user app/provider/credential execution.
