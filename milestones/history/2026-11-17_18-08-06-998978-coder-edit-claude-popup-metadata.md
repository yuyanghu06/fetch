# Add Name and Description to the Claude Edit popup

- Local timestamp / acting role / milestone: 2026-11-17T18:08:06-05:00 / Coder / Claude subagent metadata editing
- Requested scope: Support the two fields using the current Claude definition draft.
- Starting repository state: Preview displays both values, and Edit provides a raw editor.
- Work performed / changed files: src/ui/workspace.rs displays both input fields and applies changes to the current editor draft with undo. src/workspace/agents.rs uses the existing YAML parser and serializer and carries through the original instruction body. src/workspace/mod.rs exports the draft helper.
- Decisions: Keep the filename separate from the Name value. Keep the raw editor available. Settings retain their values while the YAML header formatting is normalized.
- Checks and evidence: Inspected the changes and passed git diff --check. Focused UI verification is underway.
- Resulting state / acceptance: Implementation is ready for verification.
- Remaining work and handoff: Verify editing and preview, check the Save destination, and build the signed executable.
