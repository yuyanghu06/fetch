# Measured opening size and light Markdown code implemented

- Local timestamp / acting role / milestone: 2026-11-16T13:46:31.645769-05:00 / Coder, integrated by Orchestrator / workspace-session-error
- Requested scope: default1058x789window, light fenced/inline code matching context, combined name/result tool Copy.
- Starting repository state: accepted plan from native read-only measurement and local Iced Viewer inspection.
- Work performed / changed files: Coder updates src/ui/mod.rs, components.rs, workspace.rs, theme.rs and info.md. Shared CODE_BACKGROUND feeds existing code-box and Markdown inline highlight. Supported custom Viewer renders fences through shared12px mono inset box with glyph wrap; inline code uses BODY_FONT14px/INK. Cached items and other Markdown rendering remain. Tool Copy uses name + blank line + original result.
- Decisions: opening size only; min760x520and other window policies unchanged. User explicitly requested combined Copy; original history/display cache remain separate. No new parser/dependency/API/personal-file changes.
- Checks and evidence: Coder diff check passes; root inspects Viewer and size implementation. Signed build and independent checks running.
- Resulting state / acceptance: code stable for verification.
- Remaining work and handoff: actual-widget default/minimum captures, combined Copy and Markdown regression, relevant default-size pixel fixture corrections/tests, final build and independent evaluation. No running Fetch/window mutation.
