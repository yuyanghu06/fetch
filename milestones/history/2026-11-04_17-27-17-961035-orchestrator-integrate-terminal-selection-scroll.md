# Integrate terminal interaction corrections

- Local timestamp / acting role / milestone: 2026-11-04T17:27:17.961032-05:00 / Orchestrator / terminal-selection-scroll.
- Requested scope: user-confirmed gray selection blocks and inability to scroll up in Terminal.
- Starting repository state: two original interaction failures independently reproduced, narrow view.rs fix handed off; unrelated dirty/concurrent work preserved.
- Work performed / changed files: Inspected final source/tests and actual logs; updated notes/context/source UI guide/UI-state index/attribution and milestone evidence. Only production change is vendored view.rs; tests stay in server_terminal_tests.rs. Other concurrent work remains preserved. No app relaunch, personal login/credentials/settings, commit/push or release action.
- Decisions: release outside ends active drag; pixel sign matches line wheel; visible-pane fractional gestures captured. Reuse Alacritty history/Iced; no cursor-style, parser/API/process/auth/PATH change.
- Checks and evidence: exact commands/outcomes and inspected log paths in milestones/terminal-selection-scroll/evidence.md.
- Resulting state / acceptance: scoped implementation/checks/build/review accepted.
- Remaining work and handoff: user must explicitly Quit/relaunch Fetch to observe actual corrected native window. Deliberate selection still highlights text normally; no native runtime acceptance inferred from offscreen evidence.
