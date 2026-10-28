# Record virtual testing options and proposed operating rules

- Local timestamp / acting role / milestone: October 28, 2026, 13:13:01.799126 America/New_York / Orchestrator / isolated-mcp-testing.
- Requested scope: User clarified virtual testing rather than remote hosting and asked about standard software.
- Starting repository state: Clean main at 1fae5b6; implemented developer foundation and full-MVP limits unchanged. Existing NOTES.md, architecture index and milestone journal retained.
- Work performed / changed files: Added docs/architecture/isolated-testing.md and milestones/isolated-mcp-testing/info.md; updated the existing worklist/index/folder guides, NOTES.md, milestone snapshot/to-do and history guide; serialized the inspected Researcher report and this pass. No application edits.
- Decisions: Record alternatives and recommendations as proposed, without choosing a vendor, provisioning or expanding live-test permission. Preserve D46/D74 and the single engineering worklist. Proposed tests keep Fetch/clients/apps inside the guest and use dedicated data/accounts, reviewed sharing/network access and clean test copies.
- Checks and evidence: Inspected primary-source findings and documentation diff; local-link existence and git diff --check verified for this bounded documentation pass. No application tests required; documentation checks by Orchestrator, not an independent runtime acceptance.
- Resulting state / acceptance: Comparison ready for discussion. First developer foundation stays complete; full MVP and isolated verification stay incomplete. No dependencies, credentials, clients, trust, services or environments changed.
- Remaining work and handoff: Explain the options plainly, then use the user's preferred workflow and compatible Mac to finalize the concrete testing plan before operational changes. No release or live connection test.
