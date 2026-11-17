# Accept single-card copies in toolbar Paste

- Local timestamp / acting role / milestone: 2026-11-17T17:47:38-05:00 / Planner / Workspace agent actions
- Requested scope: Use the Subagents toolbar Paste button with whatever subagent selection was copied.
- Starting repository state: Toolbar Paste reads the current clipboard but only accepts Copy all. Row Paste already accepts a single card.
- Work performed / changed files: Inspected both clipboard formats, the existing file read response, and the disposable toolbar fixture. No product files changed in this pass.
- Decisions: Accept single-card and bulk copies through the same decoder. Carry the source filename without its absolute folder. Use the existing conversion and create-only save for toolbar Paste.
- Checks and evidence: Confirmed the current parser in src/ui/workspace/bulk.rs takes only the bundle on the toolbar path. Confirmed the file read supplies the source path.
- Resulting state / acceptance: Plan accepted for the toolbar button. The row replacement flow keeps its selected destination.
- Remaining work and handoff: Implement, test a single-card copy through toolbar Paste, and review the result.
