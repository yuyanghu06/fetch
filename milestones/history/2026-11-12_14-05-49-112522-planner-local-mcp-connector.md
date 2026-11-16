# Bounded local MCP connector plan

- Local timestamp / acting role / milestone: 2026-11-12T14:05:49.112522-05:00 / Planner (connector_plan), serialized by Orchestrator / local-mcp-connector.
- Requested scope: Plan connector after research; identify meaningful API/auth choices before implementation.
- Starting repository state: Research and test inventory recorded; no connector source; extensive unrelated work retained.
- Work performed / changed files: Read-only Planner inspected CLI/gateway/OAuth/credential ownership; Orchestrator recorded bounded plan and full proposed native-approval operations in docs/architecture/local-mcp-connector.md, refreshed snapshot/to-do/guide.
- Decisions: No new user selection. Recommend fetch connect raw transport, per-client profiles, native approval option B and one-time login setup; existing browser option A remains possible with browser trust prerequisite.
- Checks and evidence: Same-grant equal request IDs and cross-process rotating-refresh race identified from source; proposed per-process IDs and file lock. Test inventory source evidence, not runtime success. Exact proposed routes/inputs/permissions/status/expiry are awaiting user decision. No code/tests/apps/client/credential/trust changes.
- Resulting state / acceptance: Complete bounded reviewable plan; independent evaluation requested next. New auth implementation remains gated by user's explicit choice.
- Remaining work and handoff: Evaluate plan, present concrete approval decision and actual-client isolation limit. After selection, implement/test/evaluate scoped work without project/release/personal-state changes.
