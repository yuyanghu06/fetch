# Add interactive local startup and private login-link reader

- Local timestamp / acting role / milestone: 2026-11-03T23:11:48.941919-05:00 / Coder A / standard-authenticate.
- Requested scope: Normal local startup for explicit login, observer before spawn, interactive wait and bounded selected URLs.
- Starting repository state: Existing accepted raw runner plus concurrent root/Manager work preserved.
- Work performed / changed files: src/runner/mod.rs, transport.rs and new local_setup.rs. Added connect_for_authentication and shared setup output reader; existing lifecycle and raw messages retained.
- Decisions: Local explicit setup gets600second initialize/discover; normal/remote stay30seconds. Reader emits only recognized HTTPS login URLs, no raw output; bounded16KiB lines/8KiB URLs and nonblocking channel. Parent owns full-attempt timeout/Quit/browser presentation.
- Checks and evidence: Root inspected diff/source; Coder self-review and owned-file git diff --check pass. Five parser/stream tests added but not run at this checkpoint. No server/account/credentials executed.
- Resulting state / acceptance: Transport source ready for combined build/independent tests; not runtime acceptance.
- Remaining work and handoff: Manager/UI/API integration, compile/tests, independent evaluation/fix/retest.
