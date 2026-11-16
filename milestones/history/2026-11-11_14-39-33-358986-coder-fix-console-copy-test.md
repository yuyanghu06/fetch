# Correct Console Copy test interaction

- Local timestamp / acting role / milestone: 2026-11-11T14:39:33.358986-05:00 / Coder / console-on-expansion.
- Requested scope: repair new actual-widget test's Copy click after independent Tester finding.
- Starting repository state: product implementation complete; first regression reached both output assertions, then failed because Copy is an icon without a text label.
- Work performed / changed files: src/ui/neumorphic_tests.rs now clicks actual Copy icon via existing frame/container bounds and mouse events; no product code changes in this pass.
- Decisions: use real shared control interaction rather than searching nonexistent text.
- Checks and evidence: root inspected correction and git diff --check passes; independent rerun pending. Root separately ran ./scripts/run-development.sh --build-only successfully (7.95s build, signing and strict verification succeeded; existing deprecation/unused warnings). Mac toolchain and selected local Code Signing identity were checked read-only beforehand.
- Resulting state / acceptance: corrected test awaits rerun; signed development executable updated, app not launched.
- Remaining work and handoff: finish Tester report, inspect image and independent Evaluator review.
