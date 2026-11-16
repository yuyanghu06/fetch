# connector review fixes

- Local timestamp / acting role / milestone: 2026-11-12T14:28:48.808530-05:00 / orchestrator, serialized by Orchestrator / local-mcp-connector.
- Requested scope: Implement selected native-popup setup/stdio bridge; independent tests/review with isolation.
- Starting repository state: Reviewed plan, source integrating, extensive unrelated work preserved.
- Work performed / changed files: Root preserves claim ownership across window close/reopen, cleans denied/expired requests even after backgrounding and leaves authority in native APIs. Coder serializes cancellation behind original dispatch receipt.
- Decisions: Existing approved scope only; no project/release/packaging/client edits, trust changes or migration.
- Checks and evidence: No new route/permission. Focused regression tests added, no new personal-state access. Real-client fixture harness compiles but first stopped at confinement probe before client execution; tester correcting harness without weakening boundary.
- Resulting state / acceptance: Intermediate pass recorded. Current review remains REQUEST CHANGES until fixes are retested and accepted; real-client/native acceptance not inferred.
- Remaining work and handoff: Complete immediate-cancel and UI/CLI regressions, real-client confinement/protocol smoke, signed build-only and final independent review. Native Keychain/Mac popup real acceptance remains unobserved.
