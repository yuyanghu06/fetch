# Persistent Fetch-only Codex — tester pass

- Local timestamp / acting role / milestone: 2026-11-04T18:17:53.486311-05:00 / tester / persistent-fetch-only-codex.
- Requested scope: Create a permanent test Codex profile and reusable launcher, with Fetch as its only configured MCP and normal Codex settings preserved.
- Starting repository state: Unrelated Python environment implementation and documentation changes already present; preserved.
- Work performed / changed files: Independent Tester checked actual scoped Codex MCP listing/features, shell parsing, source-to-config mapping, permissions, same-command login help and fake-executable argument/environment forwarding. No product/runtime source changes.
- Decisions: Dedicated client root under current HOME's Library/Application Support/Fetch-Development/clients/codex-test; authoritative .mcp.json before scoped config generation. No global sync, credential copying, live login, Fetch launch or MCP connection.
- Checks and evidence: PASS: only enabled fetch at development URL, not_logged_in; apps/plugins/remote_plugin=false. sh -n passes; file credential stores; root/workspace/sqlite/certificate and --no-daemon forwarded. No auth files; workspace empty. Old temporary root absent.
- Resulting state / acceptance: Configuration-only verification passed.
- Remaining work and handoff: User additionally requests portable setup documentation for other Macs; write guide, independently review. Actual user login and live tool use remain untested.
