# Pointer coordinates inside a scrolled page

Goal: normal terminal clicks do not turn into large text selections when the surrounding server page has been scrolled. Preserve deliberate text selection, existing cursor appearance and the prior drag-release/trackpad fixes.

Cause under verification: Iced's outer Scrollable forwards the original mouse event but adjusts its child Cursor position. The terminal press uses adjusted coordinates; mouse movement incorrectly uses the raw window event. Use the existing adjusted position consistently, without replacing selection logic.

Scope: two-line correction in vendored view.rs; one realistic actual Scrollable+TerminalView interaction/render regression. Existing terminal UI regressions/native build and independent review. No app/provider/account/credential execution; no parser/API/process/authentication changes. Coder owns view.rs, Tester owns server_terminal_tests.rs, root owns journal/docs.

Status: complete scoped correction. Actual outerScrollable regression reproduces original jump and passes after two-line fix;11terminalUI checks1new/native build6.90s/whitespace and EvaluatorACCEPT. See evidence.md. Actual rebuilt native user window remains unobserved.
