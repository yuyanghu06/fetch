# Preserve inherited command folders

- Local timestamp / acting role / milestone: 2026-11-04T17:11:43.880643-05:00 / Coder and Orchestrator / normal-program-path.
- Requested scope: preserve normal command-search folders after server-private executable folders so gh works by name.
- Starting repository state: user-approved change, shared native transport/system-only default; unrelated dirty changes and concurrent callback work preserved.
- Work performed / changed files: Coder changed only src/manager/native_state.rs: record assigned PATH, merge existing inherited environment before composing path, prefix private folders, include executable parent on default branch, then inherited PATH or existing system fallback. Root inspected actual source; credentials/HOME/XDG/setup/terminal call structure unchanged. Scoped whitespace check passes; no runtime/tests by Coder.
- Decisions: no shell profiles, guessed commands, account migration or public contract changes. Inherits the PATH available to Fetch; does not discover directories absent from it.
- Checks and evidence: source inspection and scoped git diff --check; focused independent tests/build and review underway, no pass claimed yet.
- Resulting state / acceptance: bounded plan/source ready for independent checks.
- Remaining work and handoff: Tester owns fixtures and native build, Evaluator read-only review; root owns journal/docs. No personal gh/MCP/login or app launch.
