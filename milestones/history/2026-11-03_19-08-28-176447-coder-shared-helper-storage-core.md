# Implement shared helper storage and process leases

- Local timestamp / acting role / milestone: 2026-11-03T19:08:28.176447-05:00 / Core Coder, inspected by Orchestrator / shared-mcp-authentication.
- Requested scope: Approved reusable helper credential bridge with Google first; preserve existing callbacks/public API/raw MCP and encrypted renewal.
- Starting repository state: Research/plan complete; existing unrelated UI/startup/deletion documentation/source changes preserved. No shared bridge existed.
- Work performed / changed files: Added credentials/bridge.rs and bridge_client.mjs; exported bridge/test module from credentials/mod.rs; added ManagedStdio attachment and Connection-owned lease cleanup in runner/mod.rs/transport.rs. Root inspected source/diff; Coder fixed permanent revoked-Factory fencing and authorized operation-timeout failure reporting before completion.
- Decisions: Fixed server/adapter owner, random per-process capability and0700 short private Unix socket dir/0600 socket; bounded load/update/verify with conditional helper-subsection revision and encrypted readback ACK. Runtime updates verify saved access before ACK; refresh-only login writes remain unverified. Dedicated gates avoid Manager-lock deadlock; blocking writes off async executor, revocation fences in-flight writes. Session cleanup preserves raw MCP stdin/stdout and never logs credentials. No new dependency or public product route/UI.
- Checks and evidence: Source inspection only; no build/test/helper/provider execution by Core Coder. Independent Tester's fixture implementation underway; independent Evaluator reading source.
- Resulting state / acceptance: Shared core source stable for adapter integration/testing; no runtime acceptance yet.
- Remaining work and handoff: Complete Google/Manager integration, independently test/account isolation/renewal/no fallback, compile and resolve findings. Actual provider login requires protected dedicated credentials.
