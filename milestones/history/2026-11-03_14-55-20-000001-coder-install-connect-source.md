# Implement default-on installation connection

- Local timestamp / acting role / milestone: 2026-11-03_14-55-20 America/New_York / Coder, inspected and serialized by Orchestrator / install-connect D83.
- Requested scope: Use existing Restart / Connect after preparation, retain successful connections, expose not_required and hide speculative auth controls.
- Starting repository state: Checkbox9eddd00 pushed; approved D83 plan/docs and unrelated concurrent documentation edits preserved.
- Work performed / changed files: Four owned files: installation/mod.rs, manager/mod.rs, runner/mod.rs and ui/mod.rs. Weak per-submission hook awaits named restart; shared classification retains runtime errors, no-auth result and later provider rejection; UI hides unknown/no-auth login controls.
- Decisions: Preserve prepared installs and ready reuse; credential-free success with no known login requirement is not_required, never authenticated; no automatic login/tool calls; GET servers remains status.
- Checks and evidence: Read source/diff. Locks release before restart/feature observation, callback has no persistent strong cycle, normal bounds/close guards retained. No tests added or run, builds, checks, format/lint, app/provider/MCP or personal-state actions.
- Resulting state / acceptance: Implemented source; independent Tester found one generation/connection identity race. Source not yet accepted; no runtime evidence.
- Remaining work and handoff: Fix identity binding, re-review, evaluate, update guides and deliver. Build/native/runtime acceptance remains unrun.
