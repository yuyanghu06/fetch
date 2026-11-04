# Accept inherited command lookup

- Local timestamp / acting role / milestone: 2026-11-04T17:14:41.049864-05:00 / Independent Evaluator / normal-program-path.
- Requested scope: preserve normal command folders after private server programs so gh works by name in startup/terminal.
- Starting repository state: source handed off from Coder; existing dirty tree/concurrent work preserved; focused checks and review pending.
- Work performed / changed files: Read-only review ACCEPTs shared builder change, explicit PATH override and private HOME/XDG preservation. Independently inspected23 passing checks, one real confined terminal flow, native build8.40s, whitespace and cleanup evidence. No remaining material scoped finding; user native reload/login untested.
- Decisions: inherit Fetch process PATH through the existing builder; private folders first, executable parent on default branch, assigned PATH preserved, system fallback only if absent. No shell profiles or credential migration.
- Checks and evidence: see milestones/normal-program-path/evidence.md for exact commands, outcomes and original inspected log paths.
- Resulting state / acceptance: scoped implementation/tests/build/review ACCEPT.
- Remaining work and handoff: rebuilt native app/fresh terminal and personal login not observed; user must explicitly Quit Fetch then ./scripts/run-development.sh. Inherited PATH must contain the desired global program folder; no separate discovery/profile loading selected.
