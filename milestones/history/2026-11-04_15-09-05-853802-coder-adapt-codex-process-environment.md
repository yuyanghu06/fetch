# Adapt Codex process environment

- Local timestamp / acting role / milestone: 2026-11-04T15:09:05.853802-05:00 / coder / codex-startup-environment
- Requested scope: Replace environment assembly with actual pinned Codex source while retaining Fetch protected overrides.
- Starting repository state: Concurrent terminal/auth/logging work remains dirty and outside source ownership.
- Work performed / changed files: Added src/manager/process_environment.rs; localized transport wiring in native_state.rs and helper/test module declarations in manager/mod.rs. Root inspected actual files. Exact upstream Unix/default, CA and six blocked internal names retained; parent snapshot is injectable and production reads only allowed names. Private path conversion now rejects unrepresentable UTF-8 instead of changing bytes.
- Decisions: Preserve private HOME/XDG directories and existing helper PATH before applying Codex inheritance; preserve LANG fallback only when absent. No dependency, API, authentication or service credential changes.
- Checks and evidence: Source inspection confirms recorded setup reuses transport, command/args/cwd and env_clear are unchanged; native terminal code from another owner preserved. No tests/build performed by Coder. Root retained exact pinned Apache LICENSE/NOTICE; header correction points to those root files.
- Resulting state / acceptance: Source ready; independent Tester and Evaluator active. No runtime acceptance claimed.
- Remaining work and handoff: Run pure helper/transport and confined child fixtures, native build, independent review and any corrections; finish evidence/docs.
