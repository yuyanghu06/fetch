# Fresh terminal startup query correction

- Local timestamp / acting role / milestone: 2026-11-04T15:20:59.717647-05:00 / Coder and Orchestrator / server-console-terminal.
- Requested scope: fix independent review/test finding within the approved contract.
- Starting repository state: initial independent review requests two corrections; source remains otherwise implemented.
- Work performed / changed files: src/ui/client.rs and src/ui/server_terminal.rs preserve HTTP201 versus200 as private client metadata. New shells answer early terminal device queries; only reused sessions suppress replayed replies.
- Decisions: no public route/schema/provider-auth change; preserve exact process ownership and native-only authority.
- Checks and evidence: Source inspection and git diff --check pass. New/reused regression fixtures and final current-tree checks remain with independent Tester.
- Resulting state / acceptance: correction handed off; final independent acceptance remains pending.
- Remaining work and handoff: current-tree tests, locked native build and Evaluator review. No native app or provider account launched.
