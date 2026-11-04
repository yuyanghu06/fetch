# Select removal of custom authentication restrictions

- Local timestamp / acting role / milestone: 2026-11-04T14:57:55.339924-05:00 / Orchestrator / wrapper-authentication.
- Requested scope: User explicitly says the custom check is no longer needed with wrapper design, then requests removal of it and other restrictions violating that design.
- Starting repository state: Significant existing source/documentation changes, including concurrent console work; preserve them. Current sdk.rs adds gates before official rmcp.
- Work performed / changed files: Inspected current project context, architecture contracts/open work, source, pinned rmcp 3.8.0 and development workflow; delegated bounded read-only Planner and Researcher audits. Created milestone scope and updated current records/architecture decision.
- Decisions: Remove Fetch-imposed provider authentication compatibility gates, not the library's own validation or existing application permission/network/credential boundaries. Existing callback route/API shapes unchanged; no provider-specific replacement flow, credential migration or personal settings edits.
- Checks and evidence: Pinned SDK source already makes callback issuer mandatory only when advertised, and exposes legacy endpoint fallback; Manager already forwards optional issuer directly to the library. No implementation/tests in this planning pass.
- Resulting state / acceptance: Bounded removal authorized. Planner/audit underway; source/test/review evidence pending.
- Remaining work and handoff: Complete independent plan/audit; implement, independently test/evaluate, fix/retest, integrate journal. No live-account auth or app launch required.
