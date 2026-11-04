# Persistent Fetch-only Codex — coder pass

- Local timestamp / acting role / milestone: 2026-11-04T18:17:03.511887-05:00 / coder / persistent-fetch-only-codex.
- Requested scope: Create a permanent test Codex profile and reusable launcher, with Fetch as its only configured MCP and normal Codex settings preserved.
- Starting repository state: Unrelated Python environment implementation and documentation changes already present; preserved.
- Work performed / changed files: Independent Coder created private persistent root, authoritative .mcp.json, generated config.toml, sqlite/workspace, launch.command and ~/.local/bin/fetch_codex short wrapper. Missing clients container created0700. Orchestrator inspected all settings/launcher files. Removed only /private/tmp/fetch-codex.NtT334 after its process exited and lsof confirmed no open files.
- Decisions: Dedicated client root under current HOME's Library/Application Support/Fetch-Development/clients/codex-test; authoritative .mcp.json before scoped config generation. No global sync, credential copying, live login, Fetch launch or MCP connection.
- Checks and evidence: Both shell launchers pass sh -n. New directories/launchers0700; settings0600. Config disables apps/plugins/remote_plugin and uses file credentials. No credentials, sessions or live connections copied/started.
- Resulting state / acceptance: Files created; independent configuration checks next.
- Remaining work and handoff: Tester checks actual CLI configuration and argument forwarding, then independent review. User sign-in and actual Fetch tool call remain separate.
