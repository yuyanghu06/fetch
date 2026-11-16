# Diagnose clipped tool shadow and outer window border

- Local timestamp / acting role / milestone: 2026-11-09T17:51:34.376925-05:00 / Planner (primary agent) / tool-shadow-boundary.
- Requested scope: fix pictured hard right tool edge; user also reports outer window boundary is not borderless.
- Starting repository state: prior navbar/tooltip/fixed-list changes and unrelated Figma records remain uncommitted; preserved.
- Work performed / changed files: read installed Iced Grid/Row/Column/Scrollable source and Fetch decoration/layout/window settings. Grid's draw method intersects viewport with grid bounds; current retained screenshot shows rightmost-card shadow abruptly replaced by flat padding. Added a pixel assertion to the existing actual-widget three-column/scroll test. Initial assertion edit had a variable-shadowing compile error, corrected before baseline rerun.
- Decisions: use normal Iced rows/columns at existing cell size/gaps so scroll viewport still clips while layout does not cut decorative shadows. No new dependency/vendor edit or major architecture needed. Outer-window decorations currently use defaults; changing them may remove macOS titlebar buttons, so asked user which behavior they want and continue inner-edge work independently.
- Checks and evidence: original screenshot sample just beyond right card equals plain background [234,230,223], including near its bottom shadow. Baseline test rerun in progress.
- Resulting state / acceptance: bounded inner-edge plan ready; external window style awaits user input.
- Remaining work and handoff: prove assertion fails before, implement layout, rerun focused UI/render checks and inspect screenshot. Preserve running app and native controls pending answer.
