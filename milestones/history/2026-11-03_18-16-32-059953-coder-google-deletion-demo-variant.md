# Add Google package deletion measurement

- Local timestamp / acting role / milestone: November 3, 2026 / Coder; inspected and serialized by Orchestrator / delete-server.
- Requested scope: User identified Google as the slow MCP after the first real-package demo.
- Starting repository state: Everything install/echo/delete passed; no production deletion fix. Temporary evidence retained.
- Work performed / changed files: Refactored only tests/delete_latency_demo.rs to share diagnostic cleanup/timing and added ignored google_package_prepare_and_delete_latency_demo pinned to @a-bonus/google-docs-mcp1.11.3. Updated tests/info.md.
- Decisions: Google uses Installer::submit preparation only; no Manager auto-Connect hook, Google MCP execution, browser sign-in, account access or provider calls. Each selected ignored case requires its own fresh root/process and exact filter.
- Checks and evidence: Locked no-run compile exit0 in3.65s. Orchestrator inspected actual source, confirming Google skips catalog/call/Connect. Prior Everything runtime evidence remains for the pre-refactor harness; Google runtime still pending.
- Resulting state / acceptance: Focused Google diagnostic compiled; production storage/manager/runner deletion files unchanged.
- Remaining work and handoff: Independent Tester to execute in a fresh verified sandbox, persist results, obtain final evidence acceptance and clean temporary environments.
