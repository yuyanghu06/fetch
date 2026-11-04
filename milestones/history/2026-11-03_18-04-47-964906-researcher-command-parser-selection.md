# Command parser selection

- Local timestamp / acting role / milestone: 2026-11-03T18:04:47.964906-05:00 / researcher, inspected by Orchestrator / direct-install-inputs.
- Requested scope: Add approved command/url alternatives to POST body, no UI.
- Starting repository state: Existing config-only source; other UI/startup/doc edits preserved; Google pending.
- Work performed / changed files: Read-only research compared shlex2.0.1 (already locked/cached) and shell-words1.1.1, both MIT OR Apache-2.0 and maintained. Selected shlex exact2.0.1 without runtime dependencies; split is quote-aware but not a shell-syntax validator. Direct path must reject operators/expansions/comments rather than silently interpret them, and reject credential-bearing bridge headers before identity persistence. Sources: https://docs.rs/shlex/2.0.1/shlex/fn.split.html and https://github.com/comex/rust-shlex/issues/13; https://docs.rs/shell-words/1.1.1/shell_words/fn.split.html and https://github.com/tmiasko/shell-words/issues/5. No code or package/runtime/provider execution. Orchestrator updated canonical approval and milestone records.
- Decisions: User selected the presented extension; implementation routine details stay within shared backend and no new public route or secret-write authority.
- Checks and evidence: Primary-source/source inspection only; no tests/build/provider/helper or personal-state action for this report.
- Resulting state / acceptance: Research/plan accepted, writer/test ownership assigned.
- Remaining work and handoff: Implement, independently test/evaluate, native build and scoped delivery. No UI work.
