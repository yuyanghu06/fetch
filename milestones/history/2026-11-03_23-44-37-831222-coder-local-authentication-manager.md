# Add server-owned local login through start API

- Local timestamp / acting role / milestone: 2026-11-03T23:44:37.831222-05:00 / Coder B / standard-authenticate.
- Requested scope: Local configured startup/helper login, safe consent, saved last-known status; user start-route steering and review fixes.
- Starting repository state: Prior refactor plus concurrent root UI/API/Runner changes; preserved.
- Work performed / changed files: manager/mod.rs, native_state.rs, bindings.rs, new local_auth.rs/local_auth_tests.rs. Added /restart interactive delegation, local attempt observation, saved status distinction, named missing-input guidance, owned task shutdown.
- Decisions: No provider OAuth recreation/token access. Same explicit start request opens pending validated link once after host display; background cannot start local flow. Helper/startup alone leaves account unknown. Atomic Runner idle guard required for both paths.
- Checks and evidence: Root inspected code/report. Three fake-browser/persistence unit cases written; no tests/builds/processes/credentials run by Coder. Dependency on Runner begin_authentication remains until follow-up source lands.
- Resulting state / acceptance: Manager source ready for combined compilation; review fixes integrated, not yet independently verified.
- Remaining work and handoff: Runner atomic guard, build, independent confined scenarios/HTTP/UI/remote regressions and final evaluation.
