# Bounded inherited PATH plan

- Local timestamp / acting role / milestone: 2026-11-04T17:11:43.880643-05:00 / Planner and Orchestrator / normal-program-path.
- Requested scope: preserve normal command-search folders after server-private executable folders so gh works by name.
- Starting repository state: user-approved change, shared native transport/system-only default; unrelated dirty changes and concurrent callback work preserved.
- Work performed / changed files: Read-only Planner confirms shared builder move before path composition, explicit-assignment distinction, existing executable-parent preservation, fallback and three existing assertion updates. One confined realistic program lookup/terminal scenario suffices; no broad tests or new API.
- Decisions: no shell profiles, guessed commands, account migration or public contract changes. Inherits the PATH available to Fetch; does not discover directories absent from it.
- Checks and evidence: source inspection and scoped git diff --check; focused independent tests/build and review underway, no pass claimed yet.
- Resulting state / acceptance: bounded plan/source ready for independent checks.
- Remaining work and handoff: Tester owns fixtures and native build, Evaluator read-only review; root owns journal/docs. No personal gh/MCP/login or app launch.
