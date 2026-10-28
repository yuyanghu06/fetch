# Compare isolated testing software

- Local timestamp / acting role / milestone: October 28, 2026, 13:13:01.798993 America/New_York / Researcher report serialized by Orchestrator / isolated-mcp-testing.
- Requested scope: Explain standard software for virtual testing and identify a suitable boundary for Fetch's Mac/client/MCP tests.
- Starting repository state: Clean main at 1fae5b6; first developer backend implemented/pushed, real MCP tests deferred under D46/D74.
- Work performed / changed files: Read-only specialist researched official Python, Docker/Podman, UTM, Parallels, Tart and Apple documentation; no files or runtime state changed. Orchestrator records findings in docs/architecture/isolated-testing.md and this journal.
- Decisions: No new user decision. Recommend a clean macOS VM, UTM for interactive work or Tart for automation; Parallels is a paid alternative. Dependency-only environments are insufficient for D46.
- Checks and evidence: Primary sources linked in isolated-testing.md; checked macOS guest/Apple Silicon support, container versus package isolation, sharing risks, version-dependent UTM reset features and unproven guest biometrics. No installation or runtime test.
- Resulting state / acceptance: Research inspected and usable for a proposed comparison; it does not establish vendor selection or live compatibility. Tart license must be checked against its current repository rather than stale secondary wording.
- Remaining work and handoff: User chooses workflow/environment; concrete host/image/network/accounts/reset/test matrix and operational authorization remain before provisioning/live tests. Specialist did not spawn agents.
