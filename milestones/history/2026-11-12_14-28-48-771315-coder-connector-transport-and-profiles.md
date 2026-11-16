# connector transport and profiles

- Local timestamp / acting role / milestone: 2026-11-12T14:28:48.771315-05:00 / coder, serialized by Orchestrator / local-mcp-connector.
- Requested scope: Implement selected native-popup setup/stdio bridge; independent tests/review with isolation.
- Starting repository state: Reviewed plan, source integrating, extensive unrelated work preserved.
- Work performed / changed files: Transport coder completed raw stdio/HTTPS forwarding, per-process IDs, profile Keychain namespace, cross-process refresh lock, setup launch/focus helper and CLI. Reuses existing SSE/raw/TLS/redaction code.
- Decisions: Existing approved scope only; no project/release/packaging/client edits, trust changes or migration.
- Checks and evidence: New src/connector modules, main/lib wiring, io-std feature. Seven bridge/profile tests passed after correcting fixture roots to0700; immediate-cancel follow-up is pending.
- Resulting state / acceptance: Intermediate pass recorded. Current review remains REQUEST CHANGES until fixes are retested and accepted; real-client/native acceptance not inferred.
- Remaining work and handoff: Complete immediate-cancel and UI/CLI regressions, real-client confinement/protocol smoke, signed build-only and final independent review. Native Keychain/Mac popup real acceptance remains unobserved.
