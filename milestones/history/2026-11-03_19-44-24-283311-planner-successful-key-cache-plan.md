# Plan successful-key caching until Quit

- Local timestamp / acting role / milestone: November 3, 2026 / independent Planner; serialized by Orchestrator / keychain-prompts.
- Requested scope: Reduce duplicate Keychain reads while retaining separate keys and existing encrypted storage.
- Starting repository state: Native provider reads each time; no key cache; current source and startup reviewed.
- Work performed / changed files: Read-only cache/lifecycle/backend-test seam plan. Orchestrator records plan and exclusive source ownership.
- Decisions: Private native backend module plus successful-only per-ID Zeroizing keys, one serialized gate, OS readback after create, no failure caching, delete eviction before OS result, and default provider clear hook called during ApplicationHost shutdown after manager stop. Permanently close provider to prevent retained-task refill.
- Checks and evidence: Existing provider/store/shutdown inspected. Cache tests require fake OS operations only; separate IDs/namespaces, retry, create proof, delete outcomes and retained/in-flight close.
- Resulting state / acceptance: Bounded plan accepted; no storage-format/public API/native-confirmation change.
- Remaining work and handoff: Coder implementation, independent tests/evaluation, native build and signing workflow integration.
