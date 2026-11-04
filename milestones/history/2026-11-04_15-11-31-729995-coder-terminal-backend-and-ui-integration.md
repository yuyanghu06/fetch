# Terminal backend and UI integration

- Local timestamp / acting role / milestone: 2026-11-04T15:11:31.729995-05:00 / Coders and Orchestrator / server-console-terminal.
- Requested scope: native Console/Terminal switch using the account shell and exact managed MCP environment.
- Starting repository state: explicit user-approved API/storage contract and dependencies; preserved continuous-console and other concurrent work.
- Work performed / changed files: inspected new terminal backend and URL-shaped API handler, Manager wiring, UI state/client/shared controls and external iced_term seam. Backend owns portable-pty, bounded byte output and cleanup; UI wraps APIs and existing parser/renderer. Added source folder/registry guidance and upstream attribution detail.
- Decisions: native-only Settings capability on every request; no arbitrary shell/env override. Switches preserve even exited screens; separate explicit Start terminal creates a new session. Close can interrupt blocked input; final status follows output drain.
- Checks and evidence: static source review; independent Tester currently preparing/running fixtures; independent Evaluator assigned. No current test/native-build acceptance claimed by this source pass.
- Resulting state / acceptance: source handed off; new fixtures and final review remain.
- Remaining work and handoff: fix test/review findings, native build, final evidence. Native appearance and real service login not observed.
