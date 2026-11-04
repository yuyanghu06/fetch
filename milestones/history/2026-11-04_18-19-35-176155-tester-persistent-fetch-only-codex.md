# Persistent Fetch-only Codex — tester pass

- Local timestamp / acting role / milestone: 2026-11-04T18:19:35.176155-05:00 / tester / persistent-fetch-only-codex.
- Requested scope: Create a permanent test Codex profile and reusable launcher, with Fetch as its only configured MCP and normal Codex settings preserved.
- Starting repository state: Unrelated Python environment implementation and documentation changes already present; preserved.
- Work performed / changed files: Independent Tester executed portable guide's Python setup block with disposable HOME and compared generated settings/launchers against the actual permanent setup. Fixture removed after verification.
- Decisions: Dedicated client root under current HOME's Library/Application Support/Fetch-Development/clients/codex-test; authoritative .mcp.json before scoped config generation. No global sync, credential copying, live login, Fetch launch or MCP connection.
- Checks and evidence: PASS: all generated settings and launchers match; sh -n passes; modes private; workspace empty. Repeat setup refuses overwrite; all fixture hashes unchanged. No login, model session or Fetch connection.
- Resulting state / acceptance: Portable setup reproduced successfully.
- Remaining work and handoff: Independent Evaluator ACCEPT received; Orchestrator final integration and user command handoff next.
