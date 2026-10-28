# Check lightweight client and MCP isolation

- Local timestamp / acting role / milestone: October 28, 2026, 13:25:30.445395 America/New_York / Researcher report serialized by Orchestrator / isolated-mcp-testing.
- Requested scope: Assess fresh Codex/Claude Code sessions/profiles and separate MCP installs as a lower-resource alternative to a macOS VM.
- Starting repository state: main at 1fae5b6 with prior documentation preserved; no VM or live MCP tests started.
- Work performed / changed files: Specialist read official Anthropic profile/auth/CLI/sandbox docs, the Apache-2.0 sandbox-runtime repository and known issue 755; Orchestrator separately verified official OpenAI configuration/security docs. No specialist files or runtime changes.
- Decisions: No user selection. Separate paths prevent state collisions but do not restrict OS permissions. Recommend bounded profile/process restrictions and a separate standard test user for credential/desktop tests; retain VM alternatives for unresolved boundaries.
- Checks and evidence: Primary links and qualifications recorded in isolated-testing.md. Official docs exclude MCP from ordinary shell sandbox coverage; runtime whole-process coverage requires explicit access rules. Keychain issue is reported upstream, not reproduced or treated as fixed; Apple Events may launch unsandboxed apps. The specialist did not spawn agents or run sessions/tests.
- Resulting state / acceptance: Research inspected; feasible proposal with material unproven credential/app/service limits rather than a blanket isolation guarantee.
- Remaining work and handoff: Make exact access/test scope concrete and verify disposable boundaries before authorized live connections. No account creation, personal configuration/credentials or operational permission follows from this research.
