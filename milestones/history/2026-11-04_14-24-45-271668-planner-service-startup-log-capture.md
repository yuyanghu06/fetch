# Plan one startup and sign-in log capture

- Local timestamp / acting role / milestone: 2026-11-04T14:24:45.271668-05:00 / Planner (recorded by Orchestrator) / service-startup-logs
- Requested scope: Implement returned rejection details and service output during startup.
- Starting repository state: Research completed; existing native startup capture is local stderr-only and too small; no actual provider diagnosis yet.
- Work performed / changed files: Read-only audit. Proposed central Runner per-server latest capture independent of live connection, monotonic capture IDs, own Arc per task, existing private projection. Two exclusive source writers and independent Tester assigned.
- Decisions: First-to-terminal memory capture1MiB4096lines with explicit notices, shared sanitizer. Local stderr/nonprotocol stdout/log notifications/MCP errors; setup stdoutstderr/exit; remote HTTP failures and OAuth stages. Borrowed verification connections cannot freeze parent capture. No auth flow/public route/disk-log change.
- Checks and evidence: Existing UI supports growing captured prefix. Existing protocol/stdout failure/raw forwarding remain. SDK error sources reviewed by independent Researcher. No runtime run by Planner.
- Resulting state / acceptance: Bounded plan accepted, source implementation and independent test preparation active.
- Remaining work and handoff: Complete both writers, independent confined/HTTP/OAuth/privacy tests and native build, then independent evaluation and docs.
