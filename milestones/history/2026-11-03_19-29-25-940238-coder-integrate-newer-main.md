# Preserve newer main changes during discovery integration

- Local timestamp / acting role / milestone: 2026-11-03T19:29:25.940238-05:00 / Coder and Orchestrator integration / automatic-config-discovery.
- Requested scope: deliver .codex-folder discovery; preserve newer repository changes.
- Starting repository state: accepted folder change committed locally as b8c6fc7. Push rejected because origin/main advanced from a7a6e0d to ed78aac through ten existing commits.
- Work performed / changed files: fetched and merged origin/main without committing yet. Coder resolved src/discovery/mod.rs and src/mcp/management.rs; Orchestrator resolved src/ui/mod.rs and append-only/context documentation conflicts. Both sets of history files retained unchanged.
- Decisions: retain incoming installed-row flags, shared direct/file normalization, direct installation definitions, deletion tool/API/UI and authentication improvements; preserve our default/folder/file discovery and optional override controls. No force push.
- Checks and evidence: all merge conflicts resolved/staged; git diff --check passes. Diff against origin/main retains only scoped folder/UI changes and tests/docs. Existing read-only/source acceptance predates merge, so independent merged tests/evaluation are running.
- Resulting state / acceptance: integration source ready for verification, not yet accepted as a merged result.
- Remaining work and handoff: independent integrated fixture/build checks, evaluator review, merge commit and normal push. No app or personal configuration activity.
