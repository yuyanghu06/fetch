# Plan Figma Remote unsupported import

- Local timestamp / acting role / milestone: 2026-11-04T18:04:27.293539-05:00 / Planner / figma-remote-unsupported
- Requested scope: User explicitly selected unsupported status in Fetch’s import/install screen.
- Starting repository state: Existing large modified worktree; preserved. Figma registration research complete.
- Work performed / changed files: Independent read-only Planner inspected discovery classification/capture, direct installation and UI. Orchestrator records scope in docs and journal.
- Decisions: Classifier-only exact official remote endpoint restriction, including existing trailing slash and parsed mcp-remote forms. Reuse unsupported status/error and shared UI message. Do not change supported_remote_endpoint because saved-record validation shares it. No new routes/schema.
- Checks and evidence: Source inspection only; classify/capture_definition, installation/direct.rs and UI installable/install_view establish paths. No test/runtime/provider execution.
- Resulting state / acceptance: Bounded plan accepted; implementation pending.
- Remaining work and handoff: Coder owns discovery/mod.rs and ui/mod.rs; Tester owns assigned feature tests; Orchestrator serializes docs. Verify mixed remote/Desktop import and direct-install rejection, build, independent evaluation.
