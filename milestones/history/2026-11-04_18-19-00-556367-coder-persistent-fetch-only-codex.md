# Persistent Fetch-only Codex — coder pass

- Local timestamp / acting role / milestone: 2026-11-04T18:19:00.556367-05:00 / coder / persistent-fetch-only-codex.
- Requested scope: Create a permanent test Codex profile and reusable launcher, with Fetch as its only configured MCP and normal Codex settings preserved.
- Starting repository state: Unrelated Python environment implementation and documentation changes already present; preserved.
- Work performed / changed files: Portable docs/testing/CODEX_TEST_CLIENT.md added with fresh-Mac copy/paste setup, source-first generation, same-command login/use, no-overwrite behavior and per-Mac credential/certificate instructions. Orchestrator inspected guide and updated DEVELOPMENT.md/testing info pointers, dated NOTES.md and PROJECT_CONTEXT.md.
- Decisions: Dedicated client root under current HOME's Library/Application Support/Fetch-Development/clients/codex-test; authoritative .mcp.json before scoped config generation. No global sync, credential copying, live login, Fetch launch or MCP connection.
- Checks and evidence: Coder compiled Python setup example without executing it; whitespace check passes. Guide uses Path.home(), private modes, separate root/sqlite/workspace, source JSON before generated config, current feature flags and --no-daemon.
- Resulting state / acceptance: Portable guide written; execution check and review next.
- Remaining work and handoff: Verify documented setup in disposable HOME, then evaluator review. Actual OpenAI/Fetch sign-in and live tool calls remain user follow-up.
