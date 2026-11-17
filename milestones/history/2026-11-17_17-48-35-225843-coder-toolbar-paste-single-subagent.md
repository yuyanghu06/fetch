# Feed single-card copies into toolbar Paste

- Local timestamp / acting role / milestone: 2026-11-17T17:48:35-05:00 / Coder / Workspace agent actions
- Requested scope: Accept one copied subagent in the Subagents toolbar.
- Starting repository state: The toolbar and row accepted different clipboard formats.
- Work performed / changed files: src/ui/workspace.rs writes the relative filename with the source definition. src/ui/workspace/bulk.rs reads either clipboard format and sends it through the existing save and conversion code.
- Decisions: Keep old single copies usable when their recorded card name supplies a filename. Keep row replacement and toolbar file creation in their existing save paths.
- Checks and evidence: Inspected the patch and ran git diff --check. Product fixture checks are pending.
- Resulting state / acceptance: Source change is ready for the disposable toolbar fixture.
- Remaining work and handoff: Confirm exact file creation from the single payload and confirm both the original bulk and row operations.
