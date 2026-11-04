# Codex environment build and fixtures

- Local timestamp / acting role / milestone: 2026-11-04T15:11:39.990275-05:00 / tester / codex-startup-environment
- Requested scope: Adapt actual Codex MCP process environment code while preserving protected Fetch account/helper paths and service-owned credentials.
- Starting repository state: Concurrent source changes from other tasks remain dirty; source review/fixtures prepared.
- Work performed / changed files: Independent verification passed24 integration cases and8 pure helper cases. Native cargo build --locked --bin fetch passed with existing warnings. Library-test compilation initially failed on concurrent missing runner/log_store_tests.rs, terminal/tests.rs and Desktop fixture terminals field; runner file later appeared but terminal test file remained absent. No unrelated source edited.
- Decisions: Pinned commit75424f87ca2b1c482e0509a5b1ceb7459587f396; retain actual upstream inheritance/CA/blocked-name rules with Fetch explicit private overrides. No new provider-specific startup/login behavior.
- Checks and evidence: cargo test --locked --test program_paths --test environment_paths --test service_startup_logs --test standard_authentication --test isolated_native_mcp:24/24 pass. Temporary rustc helper harness8/8 pass and removed. cargo build --locked --bin fetch PASS. Real disposable startup/setup peers ran under verified deny-default confinement; account/cwd/args/helper assertions pass. See ../codex-startup-environment/evidence.md.
- Resulting state / acceptance: 32 unique exercised checks pass; native build passes. Full library suite is not claimed. New transport unit test source exists but cannot yet run in full crate-test configuration.
- Remaining work and handoff: Once concurrent terminal test wiring settles, run focused library helper/native-storage/program-path filters. Real provider/app acceptance is separate; failed explicit Restart→Authenticate observation is unchanged. No commit/push requested/performed.
