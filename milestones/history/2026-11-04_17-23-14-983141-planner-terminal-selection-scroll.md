# Plan terminal drag and trackpad corrections

- Local timestamp / acting role / milestone: 2026-11-04T17:23:14.983141-05:00 / Planner and Orchestrator / terminal-selection-scroll.
- Requested scope: stop unwanted selection continuing after mouse release and allow upward terminal history scrolling.
- Starting repository state: user confirmed gray blocks; original vendored input code unchanged while Tester reproduces.
- Work performed / changed files: read-only Planner verified event/scroll code against installed primary Iced/Alacritty source. No application edits. Tester wrote two realistic widget/headless regression scenarios and started original-source reproduction; Coder holds edits until results.
- Decisions: active left release outside visible pane ends drag and preserves existing mouse-mode release without opening a link outside; positive pixel y accumulates positive terminal scroll delta, matching line-wheel sign; consume in-pane wheel even below one line to prevent parent page movement. Keep current cursor/intentional selection.
- Checks and evidence: iced_winit0.14.1 conversion.rs215 preserves y sign for pixel/line; iced_widget0.14.2 scrollable.rs847 consumes unhandled wheel; alacritty_terminal0.25.1 grid/mod.rs164 increases display_offset for positive delta. Independent reproduction underway, no pass claimed yet.
- Resulting state / acceptance: bounded plan accepted for requested bug fixes, no API/process/authentication change.
- Remaining work and handoff: reproduce → Coder view.rs patch → focused tests/render evidence/native build → independent Evaluator; root serializes docs.
