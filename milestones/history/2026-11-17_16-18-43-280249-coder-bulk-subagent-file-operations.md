# Implement the bulk clipboard file operations

- Local timestamp / acting role / milestone: 2026-11-17T16:18:43-05:00 / Coder / Bulk subagents
- Requested scope: Package the selected definitions and read, convert and create them using the approved Workspace routes.
- Starting repository state: Approved immediate paste and additive Save.
- Work performed / changed files: Added src/ui/workspace/bulk.rs. Copy reads each selected definition before producing a bundle. Paste checks relative paths, retains same-app text, calls the existing converter for cross-app bundles, and uses create-only Save for eligible entries.
- Decisions: Keep relative subfolders. Report converter warnings for individual review and preserve the original destination file on collision.
- Checks and evidence: Source diff and whitespace checked. Runtime verification assigned separately.
- Resulting state / acceptance: Bulk data operations ready for the toolbar.
- Remaining work and handoff: Exercise the helpers with disposable Workspace file APIs.
