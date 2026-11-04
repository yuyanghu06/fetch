# Verify clicks inside a scrolled terminal page

- Local timestamp / acting role / milestone: 2026-11-04T17:41:23.628273-05:00 / Independent Tester / terminal-pointer-coordinates.
- Requested scope: recurring huge selection on normal clicking in Terminal.
- Starting repository state: actual parent-scroll coordinate mismatch reproduced; narrow two-line fix handed off; prior tests had not exercised this parent-input path.
- Work performed / changed files: One new actual Iced Scrollable+TerminalView regression failed on original source (-474 vs26 after500pxpage offset), then passes after fix. Normal tinyclick movement leaves rendered text unselected; intentional shortdrag highlights only clicked row; plainclickclears. Final11terminalUI checksPASS0.65s; native build6.90s and whitespacePASS. Changed testfileonly; no nativeapp/shell/provider/credentials.
- Decisions: use existing parent-adjusted Cursor consistently for mouse press/movement, preserve underlying selection/style/history and prior corrections. No parser/API/process/authentication/PATH change.
- Checks and evidence: exact commands/outcomes and inspected log paths in milestones/terminal-pointer-coordinates/evidence.md.
- Resulting state / acceptance: scoped source/checks/build/review accepted.
- Remaining work and handoff: user explicitly Quit/relaunch Fetch to load the correction and observe native clicks. Do not claim user runtime acceptance from headless rendering; user screenshot login/account/device-code data not retained in fixtures/docs.
