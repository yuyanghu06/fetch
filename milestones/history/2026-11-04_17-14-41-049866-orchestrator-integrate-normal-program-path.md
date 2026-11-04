# Integrate normal command-search folders

- Local timestamp / acting role / milestone: 2026-11-04T17:14:41.049864-05:00 / Orchestrator / normal-program-path.
- Requested scope: preserve normal command folders after private server programs so gh works by name in startup/terminal.
- Starting repository state: source handed off from Coder; existing dirty tree/concurrent work preserved; focused checks and review pending.
- Work performed / changed files: Inspected final source/test files and saved logs, incorporated independent ACCEPT, updated canonical startup contract/source guide, notes/context and rolling evidence. Source change confined to native_state::transport; unrelated concurrent identity/callback work preserved. No personal settings/credentials or running app/services changed; no commit/push/release. Fresh terminal after explicit Quit/signed relaunch is next user action.
- Decisions: inherit Fetch process PATH through the existing builder; private folders first, executable parent on default branch, assigned PATH preserved, system fallback only if absent. No shell profiles or credential migration.
- Checks and evidence: see milestones/normal-program-path/evidence.md for exact commands, outcomes and original inspected log paths.
- Resulting state / acceptance: scoped implementation/tests/build/review ACCEPT.
- Remaining work and handoff: rebuilt native app/fresh terminal and personal login not observed; user must explicitly Quit Fetch then ./scripts/run-development.sh. Inherited PATH must contain the desired global program folder; no separate discovery/profile loading selected.
