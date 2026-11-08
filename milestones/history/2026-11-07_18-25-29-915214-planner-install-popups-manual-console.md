# Plan Install page popups and manual console

- Local timestamp / acting role / milestone: 2026-11-07T18:25:29.915214-05:00 / planner / neumorphic-figma
- Requested scope: Move all discovery outcomes into popups and add a manual-install section with console.
- Starting repository state: Earlier neumorphic Figma baseline and icon pass complete; existing unrelated changes preserved.
- Work performed / changed files: Read-only Planner inspected the Install screenshot/source and recommended a centered dialog with choices/select-all/fixed install footer, and a separate manual input/button/shared Console.
- Decisions: Keep this pass bounded to the user-selected presentation changes; no API or authentication behavior change.
- Checks and evidence: Preserve disabled/installed reasons, selections, progress and failures; root extended the plan to include searching and empty dialogs per the user.
- Resulting state / acceptance: Bounded Figma plan selected.
- Remaining work and handoff: Do not infer manual command execution/API behavior or implement it in Rust.
