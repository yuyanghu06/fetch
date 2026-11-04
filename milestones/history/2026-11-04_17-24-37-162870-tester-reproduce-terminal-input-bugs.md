# Reproduce terminal selection and trackpad failures

- Local timestamp / acting role / milestone: 2026-11-04T17:24:37.162870-05:00 / Independent Tester and Orchestrator / terminal-selection-scroll.
- Requested scope: reproduce unwanted selection and inability to scroll up before editing original widget.
- Starting repository state: Planner verified source causes; Coder held production edits for independent reproduction.
- Work performed / changed files: Tester added two realistic widget/headless interaction checks in src/ui/server_terminal_tests.rs. Root inspected tests and original output. No production source change during reproduction.
- Decisions: actual mouse events drive the existing widget/parser/renderer; preserve normal intentional selection and unchanged white cursor. Trackpad scenario checks fractional capture, old-output visibility, return toward latest and empty-poll stability in the same user flow.
- Checks and evidence: /private/tmp/fetch-terminal-selection-scroll-before.log: selected3tests,1existingpass/2newfail. Selection emits SelectUpdate after release outside and reentry; fractional wheel over pane is not captured. Tests establish both reported interaction failures. Original build49.79s; test0.83s.
- Resulting state / acceptance: concrete failures reproduced; Coder hold released. No corrected-source pass claimed yet.
- Remaining work and handoff: Coder view.rs fix, independent retest/render checks/native build and Evaluator review. No app/shell/MCP/provider/credentials involved.
