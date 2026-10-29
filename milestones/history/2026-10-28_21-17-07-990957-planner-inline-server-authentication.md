# Plan inline authentication in installed server cards

- Local timestamp / acting role / milestone: 2026-10-28T21:17:07.991426-04:00; Planner; inline-server-authentication.
- Requested scope: user screenshot shows Railway Authenticate yielding a generic Saved notice; user requires authentication flows inline.
- Starting repository state: unrelated Figma/shared-component documentation changes are present; existing native source uses generic Changed completion for Authenticate.
- Work performed / changed files: Planner inspected src/ui/mod.rs, client.rs, manager authentication handlers and accepted API contract read-only. Orchestrator serialized this report and created milestone scope/current-state entries.
- Decisions: preserve existing API/provider support and Settings-only manual credentials. Implement dedicated per-server UI states, shared credential renderer, polling, stale/duplicate guards and inline results. Railway remains unsupported rather than fake success.
- Checks and evidence: source inspection shows only remote Linear opens browser OAuth; unknown/required responses are currently discarded into Saved. Shared personal role/workflow files are absent; specialists use inherited runtime configuration, no new role settings created. No tests/build/launch/network/credentials actions.
- Resulting state / acceptance: bounded plan ready, Coder owns src/ui/mod.rs and src/ui/authentication.rs only; root owns all documentation/journal writes.
- Remaining work and handoff: implement, independently inspect source/edge cases, compile binary only, evaluate/fix and integrate. No live authentication or tests authorized.
