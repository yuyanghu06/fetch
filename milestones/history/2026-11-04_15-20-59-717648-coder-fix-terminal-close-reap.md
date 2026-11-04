# Terminal close/reap correction

- Local timestamp / acting role / milestone: 2026-11-04T15:20:59.717647-05:00 / Coder and Orchestrator / server-console-terminal.
- Requested scope: fix independent review/test finding within the approved contract.
- Starting repository state: initial independent review requests two corrections; source remains otherwise implemented.
- Work performed / changed files: src/terminal/mod.rs observes/reaps the shell after SIGHUP before deciding whether more signals are needed. It checks the PTY current foreground group and accepts confirmed process exit after a signal race, while retaining real failure reporting for live owned targets.
- Decisions: no public route/schema/provider-auth change; preserve exact process ownership and native-only authority.
- Checks and evidence: Source inspection and git diff --check pass. Two confined close fixtures are assigned for independent retest; no passing outcome claimed yet.
- Resulting state / acceptance: correction handed off; final independent acceptance remains pending.
- Remaining work and handoff: current-tree tests, locked native build and Evaluator review. No native app or provider account launched.
