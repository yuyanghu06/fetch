# Implement raw shared runner and native MCP source

- Local timestamp / acting role / milestone: 2026-10-28T14:20:16.098752-04:00 / Coder, source inspected by Orchestrator / mcp-mvp-implementation.
- Requested scope: Three transports, raw catalogs/results, queues/deadlines/ownership and selected native MCP methods, with no testing.
- Starting repository state: fc9b1ae installer increment pushed; authority source already present.
- Work performed / changed files: src/runner/ and src/mcp/ plus module/dependency declarations; stdio/HTTP/SSE connections, raw JSON/pagination/rejections/aliases, scheduler, progress/interactions/continuations, resources/prompts/completion and descriptive management tools. Source refinements cover explicit revocation, REST and MCP call ownership, shutdown admission and cached observation projection.
- Decisions: Native reads/get/completion use server scheduler/default duration. Keep Tasks/live content subscriptions deferred; preserve unknown raw provider fields.
- Checks and evidence: Source reading and specialist report only. No tests, build/check/lint/fmt, validation scripts, apps/services, package commands or connections. Cargo.lock resolution deferred.
- Resulting state / acceptance: Runner/native source integrated for later user-controlled verification; no runtime or client acceptance.
- Remaining work and handoff: Wire manager/HTTP/native UI; user later verifies both protocol revisions, owner/cancellation behavior and real clients.
