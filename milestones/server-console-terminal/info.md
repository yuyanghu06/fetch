# Expanded Console and system-shell Terminal

Goal: always-visible expanded console, Console/Terminal pane switch using the user’s system shell in the MCP’s own environment, and persistent sanitized console history from startup through authentication. Service retains credential ownership.

User authorized the feature/system shell. Exact new native-only terminal API and saved-log policy are approved in docs/architecture/server-terminal.md after explicit user route review. Research recommends wrapping iced_term with a narrow external-transport seam plus backend portable-pty. Source is implemented across three exclusive owners; final close/output review and independent tests are underway. Dependencies are prepared. Existing log source work remains preserved.

Completion: approved route/storage contract, independent PTY/log persistence/UI/privacy/lifecycle checks, build and evaluation. No real service login or global-account migration selected.

Status: scoped implementation and independent verification complete;182relevant checks19new/native build/whitespace pass and EvaluatorACCEPT. Actual offscreen display verifies blank-pane fix. Rebuilt native window/account login remain unobserved. One transient ordinary server read retained in evidence; rerun/five repeats pass, cause unconfirmed. See evidence.md.
