# Record Railway native callback selection

- Local timestamp / acting role / milestone: 2026-10-28T21:43:01.588828-04:00; Orchestrator; inline-server-authentication.
- Requested scope: user selects the proposed temporary numeric-loopback listener for Railway sign-in.
- Starting repository state: Railway OAuth engine source is provisional because existing HTTPS localhost callback is not evidenced for native registration; logging/UI/source fixes exist.
- Work performed / changed files: recorded D80 in architecture decisions, AGENTS.md, context/notes and current milestone/task records. Coder assigned callback implementation in manager source; no listener executed.
- Decisions: 127.0.0.1 HTTP only, random port, pending sign-in only, ten-minute original deadline, close after completion/expiry. Product APIs remain HTTPS.
- Checks/evidence: exact async user answer selected recommended listener. Earlier official Railway native quickstart is evidence for its need. No tests/build/live/auth/personal changes in this decision-record pass.
- Resulting state / acceptance: architecture choice accepted; source implementation underway. No permission re-request for the same choice is needed.
- Remaining work: implement, independently inspect lifecycle/security, compile and resolve findings. Provider/native runtime proof remains unrun.
