# Project approvals and connector profiles

- Local timestamp / acting role / milestone: 2026-11-12 17-03-55-549191 America/New_York / Auth/Connector Coder / project-connections.
- Requested scope: Implement approved exact project resources, explicit history consent and separate saved connector contexts.
- Starting repository state: Internal Caller helpers existed; OAuth and connector semantics were global-only.
- Work performed / changed files: Updated authority/OAuth/native requests/browser approval/profile/connector/application/CLI. Same immutable resource survives code exchange/refresh/reopen. Added typed project activation, separate tuple profiles/pending proofs and --project transport endpoint; application injects common Workspace service.
- Decisions: No credential copying, client file edits or global profile key migration. History initially unchecked in both approval screens. Preserve concurrent modern subscription remapping/cancellation fixes.
- Checks and evidence: Project browser/native/profile checks pass; full connector20 pass0.41s, native OAuth4 pass0.16s and final project reopen/revoke rerun pass0.10s. Missing Bridge fixture endpoint fields and private fixture root modes were corrected; real-grant connector fixture gained its missing durable Shared peer record without weakening auth or original assertion.
- Resulting state / acceptance: Implemented source and fake-key acceptance; actual Keychain/client approvals not exercised.
- Remaining work and handoff: Final signed build/evaluation recorded separately.

