# Build Install page popups and manual console

- Local timestamp / acting role / milestone: 2026-11-07T18:25:29.915215-05:00 / designer / neumorphic-figma
- Requested scope: Implement the selected Figma-only Install layout and gray already-installed entries.
- Starting repository state: Earlier neumorphic Figma baseline and icon pass complete; existing unrelated changes preserved.
- Work performed / changed files: Added shared Discovery file selector121:1450, Manual MCP installation120:1384 and Discover/install dialog set120:1519 (Results/Installing/Failed/Searching/Empty). Six Install specimens share the background; five show editable900×600dialogs. Reused actual shared checkboxes/buttons/Console; body scrolls with title/footer outside it. Close× returns to before-discovery frame; illustrative install link reaches progress. Fixed accidentally inherited gear on close controls. Disabled selected checkbox75:2284 now has gray face#E0E3E6, gray check and muted label across all three populated popup states.
- Decisions: Keep this pass bounded to the user-selected presentation changes; no API or authentication behavior change.
- Checks and evidence: Node counts105local main definitions/13sets; six background compositions/five modal variants. Root inspected popup/main-page screenshots. Final independent UX review pending.
- Resulting state / acceptance: Figma implementation complete, review pending.
- Remaining work and handoff: Manual installation execution remains unspecified/unimplemented; current Rust only contains the explicit auth-header presentation fix.
