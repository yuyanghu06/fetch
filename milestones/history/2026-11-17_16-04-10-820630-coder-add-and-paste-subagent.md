# Implement the Add subagent button and popup Paste

- Local timestamp / acting role / milestone: 2026-11-17T16:04:10-05:00 / Coder / Add subagent
- Requested scope: Match the white Edit style, place the button before the logos, open a blank copy of the existing popup and place Paste before Copy.
- Starting repository state: The Workspace form and popup already edit saved agents, and the code has a conversion path for agent definitions copied from the list.
- Work performed / changed files: workspace.rs adds the Subagents header action and focuses the blank editor, derives the destination from the chosen filename and agent directory, and connects Save to the approved create-only request. components.rs shares the raised clipboard button style and passes an ID through to the popup editor for focus.
- Decisions: An ordinary paste uses the native editor. A definition from the list Copy action can use the existing converter when pasted into a new draft. Existing popup Copy remains plain text.
- Checks and evidence: Reviewed source and ran git diff --check. Focused widget and HTTP checks are pending.
- Resulting state / acceptance: The requested UI is implemented and ready for verification.
- Remaining work and handoff: Check control order, focus, paste, rename after collision, and Save reachability at the minimum window. Rebuild and sign after verification.
