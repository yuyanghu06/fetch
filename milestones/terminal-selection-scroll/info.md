# Terminal text selection and scroll

Goal: fix the user-confirmed unwanted gray text-selection blocks and inability to scroll up in the embedded terminal. Preserve intentional selection and the existing text cursor style.

Scope: narrow vendored widget mouse/wheel handling. A left-button release outside the pane must end the active selection drag. Verify/fix pixel-wheel direction and consume in-pane wheel gestures so small trackpad movements reach terminal history rather than the outer page. Reuse Alacritty history and existing Iced input/rendering. No process/authentication/account/PATH/API changes.

Completion: reproduce the concrete interaction failures; two focused event/render tests covering selection release/re-entry and upward history scrolling, existing terminal render regressions, native build and independent review. No native app launch or personal credentials needed. Root owns journal; Coder owns view.rs, Tester owns server_terminal_tests.rs; Planner/Evaluator read-only.

Status: complete scoped fix. Original2failures reproduced; final10focused terminalUI tests2new/native build8.30s/whitespace pass, independent EvaluatorACCEPT. See evidence.md. Actual rebuilt native window remains a user runtime check after explicit Quit/relaunch.
