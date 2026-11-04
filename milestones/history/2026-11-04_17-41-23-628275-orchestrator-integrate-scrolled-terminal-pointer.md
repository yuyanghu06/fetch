# Integrate scrolled-page click correction

- Local timestamp / acting role / milestone: 2026-11-04T17:41:23.628273-05:00 / Orchestrator / terminal-pointer-coordinates.
- Requested scope: recurring huge selection on normal clicking in Terminal.
- Starting repository state: actual parent-scroll coordinate mismatch reproduced; narrow two-line fix handed off; prior tests had not exercised this parent-input path.
- Work performed / changed files: Inspected actual source and final output logs, accepted independent review, updated source guide/UI-state coverage/attribution/notes/context and milestone evidence. Only production change is two routing lines in vendored view.rs. Earlier release/trackpad fixes remain; unrelated dirty work preserved. No native app/service shutdown, credentials/personal settings, commit/push or release.
- Decisions: use existing parent-adjusted Cursor consistently for mouse press/movement, preserve underlying selection/style/history and prior corrections. No parser/API/process/authentication/PATH change.
- Checks and evidence: exact commands/outcomes and inspected log paths in milestones/terminal-pointer-coordinates/evidence.md.
- Resulting state / acceptance: scoped source/checks/build/review accepted.
- Remaining work and handoff: user explicitly Quit/relaunch Fetch to load the correction and observe native clicks. Do not claim user runtime acceptance from headless rendering; user screenshot login/account/device-code data not retained in fixtures/docs.
