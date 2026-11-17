# Carry the visible subagent name with the clipboard entry

- Local timestamp / acting role / milestone: 2026-11-17T16:44:09-05:00 / Coder / Subagent card name
- Requested scope: Reuse the source card name in single and bulk cross-app conversion.
- Starting repository state: Clipboard entries held the source app and raw content. Bulk entries also held relative filenames.
- Work performed / changed files: workspace.rs and workspace/bulk.rs copy the card name from the matching descriptor, carry it separately from file paths, and send it to the existing converter.
- Decisions: Maintain raw source text and use the original relative bulk filename. Older clipboard entries remain readable.
- Checks and evidence: Source diff and whitespace checks passed. Focused disposable API checks are running.
- Resulting state / acceptance: Both clipboard paths are wired.
- Remaining work and handoff: Verify a registered Codex label that differs from the filename and the other converter behavior.
