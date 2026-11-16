# Connector catalog latency source findings

- Local timestamp / acting role / milestone: 2026-11-12T15:34:40.718087-05:00 / Orchestrator / local-mcp-connector
- Requested scope: Explain why MCP connection is slow.
- Starting repository state: User-manual Claude connects but tool fetching fails; existing isolated source acceptance does not cover that failure. Other tasks and dirty work retained.
- Work performed / changed files: Read src/mcp/mod.rs and src/runner/mod.rs; journal/evidence only. Source inspection confirms three avoidable costs in tools/list: gateway list_tools awaits each server catalog sequentially; Runner.catalog re-requests upstream tools/list rather than using cached_catalog; each exported tool calls Runner.alias, which rewrites and synchronizes the full aliases file and containing directory even for existing mappings. collect gives its response wait a ten-second overall deadline per server (preflight/start can add further wait), so multiple slow servers can accumulate. This establishes a latency-prone code path, not measured attribution of the user's live connection closure. No source/timeout/catalog/auth behavior changed. Next: measure per-server collection and alias persistence using disposable fixtures, then review a bounded repair. Live client failure remains unresolved.
- Decisions: No new API/auth/catalog/timeout changes selected.
- Checks and evidence: Source inspection only; no test/app/provider/client/credential/trust/settings operations.
- Resulting state / acceptance: Three concrete source inefficiencies identified; exact live failure cause unconfirmed.
- Remaining work and handoff: Measure before changing behavior, preserve raw tool definitions and permissions.
