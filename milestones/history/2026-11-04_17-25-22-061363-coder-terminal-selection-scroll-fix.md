# Fix mouse release and trackpad event handling

- Local timestamp / acting role / milestone: 2026-11-04T17:25:22.061363-05:00 / Coder and Orchestrator / terminal-selection-scroll.
- Requested scope: user-confirmed unwanted gray selection and cannot scroll up.
- Starting repository state: independent two-test original-source failure established; Coder hold released.
- Work performed / changed files: only vendor/iced_term/src/view.rs modified by Coder. Pixel y accumulation now matches line wheel. Widget accepts active left release outside visible pane, ends drag and relays existing mouse-mode release without opening links outside. Visible-pane wheel events are captured even before a line accumulates; hit test intersects viewport. Root inspected actual changed source.
- Decisions: keep underlying Alacritty scroll/history and normal selection/current white cursor. No parser, process/API/auth/PATH or provider changes.
- Checks and evidence: scoped/root git diff --check PASS. Independent final10terminalUI tests including2new actual interaction/render checks are running; native build/review next.
- Resulting state / acceptance: corrected source ready, no final pass claimed yet.
- Remaining work and handoff: final focused checks/build/Evaluator and documentation. No native app launch or credentials.
