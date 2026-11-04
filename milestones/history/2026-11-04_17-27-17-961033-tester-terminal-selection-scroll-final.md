# Verify selection and trackpad corrections

- Local timestamp / acting role / milestone: 2026-11-04T17:27:17.961032-05:00 / Independent Tester / terminal-selection-scroll.
- Requested scope: user-confirmed gray selection blocks and inability to scroll up in Terminal.
- Starting repository state: two original interaction failures independently reproduced, narrow view.rs fix handed off; unrelated dirty/concurrent work preserved.
- Work performed / changed files: Two new actual-widget/headless checks reproduced original faults; final10terminalUI tests pass. Actual rendered selection stays bounded after release outside/reentry. Fractional pixel gestures captured; positive scrolling reveals earlier colored output, empty polls retain scroll, negative scrolling returns toward latest. Existing renderer/state/replay/close checks pass. Locked native build8.30s and whitespace pass. No native window/shell/provider/credentials/task processes or fixtures.
- Decisions: release outside ends active drag; pixel sign matches line wheel; visible-pane fractional gestures captured. Reuse Alacritty history/Iced; no cursor-style, parser/API/process/auth/PATH change.
- Checks and evidence: exact commands/outcomes and inspected log paths in milestones/terminal-selection-scroll/evidence.md.
- Resulting state / acceptance: scoped implementation/checks/build/review accepted.
- Remaining work and handoff: user must explicitly Quit/relaunch Fetch to observe actual corrected native window. Deliberate selection still highlights text normally; no native runtime acceptance inferred from offscreen evidence.
