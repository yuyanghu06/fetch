# Approve inherited command search path

- Local timestamp / acting role / milestone: 2026-11-04T17:10:05.626636-05:00 / Orchestrator / normal-program-path.
- Requested scope: user says yes please change that after proposal to keep normal command-search folders and prefix server-private folders.
- Starting repository state: heavily dirty tree with accepted Console/Terminal and unrelated work preserved. Existing native transport replaces host PATH with executable parent and /usr/bin,/bin unless assigned PATH exists.
- Work performed / changed files: inspected shared transport/environment builder and existing tests; assigned read-only Planner; recorded bounded scope and milestone.
- Decisions: inherit Fetch process PATH for default command lookup; private program folders stay first, explicit assigned PATH and existing private account folders remain. No shell-profile execution or personal login test.
- Checks and evidence: source inspection; current coding environment includes /opt/homebrew/bin. No new tests/build yet.
- Resulting state / acceptance: implementation authorized; no new route or credential behavior.
- Remaining work and handoff: Planner → Coder → independent focused Tester → Evaluator, fixes as needed; root serializes journal.
