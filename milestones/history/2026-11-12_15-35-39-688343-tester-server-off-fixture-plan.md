# Plan focused confined server off verification

- Local timestamp / acting role / milestone: 2026-11-12T15:35:39.688343-05:00 / Tester, parent-serialized journal / server-on-off.
- Requested scope: independently choose a few real-feature tests and exclusive files before stop implementation.
- Starting repository state: accepted persistent-off contract/plan; no stop implementation tests yet.
- Work performed / changed files: read-only inventory of existing fixture and UI helpers; parent journals the report. Tester owns new tests/server_power.rs, new src/manager/power_tests.rs and src/ui/neumorphic_tests.rs; Coder owns module declarations and all production files.
- Decisions: Tester independently selects the existing confined Sandbox + login_peer.py pattern for real HTTP/MCP stop/start fixtures; private power tests cover disabled background/conflict paths, and actual-widget tests cover native toggle pending/error/state. Coder now implements production files while Tester prepares its exclusive test files; test execution waits for the implementation checkpoint.
- Checks and evidence: source inspection only. Existing tests/native_state/confinement.rs Sandbox and login_peer.py accept dedicated disposable homes and held calls; no personal credentials/services or real MCP providers.
- Resulting state / acceptance: small test plan accepted; no tests, processes or builds run by Tester during inventory.
- Remaining work and handoff: prepare tests independently; execute after Coder is ready; parent serializes test results then Evaluator reviews actual code/evidence.
