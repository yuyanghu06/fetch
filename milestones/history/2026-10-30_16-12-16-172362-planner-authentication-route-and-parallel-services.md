# Inspect the authentication route and parallel-service limitations

- Local timestamp / acting role / milestone: 2026-10-30T16:12:16.172362-04:00 / Planner / local-working-mvp.
- Requested scope: Explain the original POST authentication check, one-key limitation and parallel authentication; produce a bounded implementation summary.
- Starting repository state: main at fe916e7 with clean working tree; prior build/68-fixture evidence only.
- Work performed / changed files: Read-only specialist inspected docs/architecture/api-contract.md, src/manager/mod.rs, src/api/runtime.rs and src/ui/mod.rs. No files changed by the specialist.
- Decisions: Preserve D25 POST stored-credential authenticated check and D31 checked credential saves. Do not infer an approved storage-only deletion exception.
- Checks and evidence: Source inspection found the set.len()==1 restriction in credentials PATCH, per-server backend locks, and a single UI auth_request plus global busy flag. Pending browser flows can overlap once their initial HTTP responses return. No commands ran tests, builds, apps, providers or MCPs.
- Resulting state / acceptance: Planner supports per-server UI request tracking and verification of actual required credential groups; no implementation or runtime acceptance.
- Remaining work and handoff: Orchestrator to explain the route distinction, record the plan and preserve unresolved deletion/protected-MCP choices. Future checks must cover parallel outcomes, same-server duplicates and stale stored-key evidence. No specialists spawned by the Planner.
