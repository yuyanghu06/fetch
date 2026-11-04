# Verify inherited command lookup

- Local timestamp / acting role / milestone: 2026-11-04T17:14:41.049864-05:00 / Independent Tester / normal-program-path.
- Requested scope: preserve normal command folders after private server programs so gh works by name in startup/terminal.
- Starting repository state: source handed off from Coder; existing dirty tree/concurrent work preserved; focused checks and review pending.
- Work performed / changed files: Changed only native_storage_tests.rs and tests/program_paths.rs. Updated old default-PATH expectations and added one actual confined PTY flow using shared transport with a child-only disposable inherited PATH. Bare gh runs a fake global program; after installing a private gh, the private program wins. No real gh/account/credentials/app executed.23 checks pass (13native_storage,8environment,2startup/setup); locked fetch build8.40sPASS; owned PTY and fixture folders cleaned.
- Decisions: inherit Fetch process PATH through the existing builder; private folders first, executable parent on default branch, assigned PATH preserved, system fallback only if absent. No shell profiles or credential migration.
- Checks and evidence: see milestones/normal-program-path/evidence.md for exact commands, outcomes and original inspected log paths.
- Resulting state / acceptance: scoped implementation/tests/build/review ACCEPT.
- Remaining work and handoff: rebuilt native app/fresh terminal and personal login not observed; user must explicitly Quit Fetch then ./scripts/run-development.sh. Inherited PATH must contain the desired global program folder; no separate discovery/profile loading selected.
