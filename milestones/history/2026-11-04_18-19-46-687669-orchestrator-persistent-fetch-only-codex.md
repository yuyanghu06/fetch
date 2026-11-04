# Persistent Fetch-only Codex — orchestrator pass

- Local timestamp / acting role / milestone: 2026-11-04T18:19:46.687669-05:00 / orchestrator / persistent-fetch-only-codex.
- Requested scope: Create a permanent test Codex profile and reusable launcher, with Fetch as its only configured MCP and normal Codex settings preserved.
- Starting repository state: Unrelated Python environment implementation and documentation changes already present; preserved.
- Work performed / changed files: Integrated permanent external client setup, short login/start command, safe removal of exited old temp client, portable guide and development/context/notes pointers. Preserved unrelated Python implementation and all existing user state.
- Decisions: Dedicated client root under current HOME's Library/Application Support/Fetch-Development/clients/codex-test; authoritative .mcp.json before scoped config generation. No global sync, credential copying, live login, Fetch launch or MCP connection.
- Checks and evidence: Independent Tester: actual configuration-only list exactly Fetch/not_logged_in, apps/plugins/remote_plugin=false, shell/modes/forwarding PASS; portable guide reproduced in disposable HOME with exact match and no-overwrite PASS. Independent Evaluator ACCEPT. Final scoped git diff --check passes; fetch_codex resolves; guide exists; old temp root absent.
- Resulting state / acceptance: Complete: permanent client and portable setup guide ready.
- Remaining work and handoff: Open a new Terminal tab, run fetch_codex login, then fetch_codex mcp login fetch with development Fetch running, then fetch_codex. Check /mcp. These real sign-ins/tool calls were not performed here; use required isolated test environment for live MCP experiments.
