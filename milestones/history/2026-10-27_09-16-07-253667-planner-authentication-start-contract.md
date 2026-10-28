# Plan the authentication-start contract

- Local timestamp / acting role / milestone: 2026-10-27T09:16:07.253667-04:00 / Planner (serialized by Orchestrator) / mcp-mvp-architecture.
- Requested scope: Bound the next existing authenticate POST contract after shared next-steps/GET approval.
- Starting repository state: Documentation-only project; D20 restart and D21 route placement approved, ordinary GET awaiting the user's final clarification/approval. Implementation paused.
- Work performed / changed files: Read current route/decision/GET/auth caveats read-only. Proposed input/permission/prerequisites, truthful known/unknown branches, one active flow, repeats/deadline/restart overlap, safe errors and current GET progress. No specialist writes.
- Decisions: All new route defaults remain proposals. Orchestrator retained the existing actor/action/instructions response shape with root server target, avoiding redundant per-step IDs for this single-server response. Concrete auth mechanisms stay step 4.
- Checks and evidence: Planner report inspected. Acceptance scenarios cover truthful branches, concurrent repeats, expiry/callback races, secret-free validation, disconnection/restart/Quit. No app tests.
- Resulting state / acceptance: Bounded proposal ready for documentation evaluation; no implementation.
- Remaining work and handoff: Review the whole proposed contract with the user, including 10-minute/provider deadline and restart overlap.

Append this entry once under `milestones/history/` with a chronological,
collision-safe filename. Corrections belong in a new entry.
