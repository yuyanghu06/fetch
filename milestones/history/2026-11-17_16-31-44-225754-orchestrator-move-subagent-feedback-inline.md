# Move subagent feedback beside its actions

- Local timestamp / acting role / milestone: 2026-11-17T16:31:44-05:00 / Orchestrator / Subagent feedback
- Requested scope: Show bulk results at the clipboard toolbar and draft errors beside Save with no checkbox.
- Starting repository state: Shared compact error and stable clipboard surfaces were ready.
- Work performed / changed files: workspace.rs keeps bulk feedback separate from page errors. Footer combines individual converter issues and Save errors behind the info marker. The smallest toolbar uses closer spacing with a reserved feedback slot.
- Decisions: Exact success messages only on success. Keep create-only collision safety, converter issues, and required-field validation. Save no longer requires acknowledgement.
- Checks and evidence: Source diff and whitespace checked. Focused tests running.
- Resulting state / acceptance: UI behavior implemented.
- Remaining work and handoff: Verify hover details, smallest layout, Save without a checkbox and no disabled press drag.
