# Plan the verified credential-save amendment

- Local timestamp / acting role / milestone: 2026-10-27T11:20:31.593615-04:00 / planner / mcp-mvp-architecture
- Requested scope: User requires endpoint test with new stored credential before credential-save200; then asks same failure code from earlier authenticate and permits moving on.
- Starting repository state: Credentials draft returned storage-only200; D25 already gated auth success; no implementation. Root sole writer, specialist read-only.
- Work performed / changed files: Inspected narrow Planner amendment identifying save/readback/new-credential proof, failed-store distinction, saved-before-failed-test outcome and verifier/edge-case gaps. Specialist changed no files.
- Decisions: Root applies one shared CREDENTIAL_VERIFICATION_FAILED per latest user request instead of Planner’s alternate split failure enums. No verification HTTP mapping/deadline/helper/restart/batch/removal/retention default silently chosen.
- Checks and evidence: Read-only contract analysis against D25/D30 and accepted auth/GET behavior. No live credential/provider/MCP tests or source changes.
- Resulting state / acceptance: Required amendment integrated with explicit remaining integration/error policy gaps; no unchecked200 exception selected.
- Remaining work and handoff: Evaluate cross-route/shared-code consistency; concrete policies must be reviewed before implementation.
