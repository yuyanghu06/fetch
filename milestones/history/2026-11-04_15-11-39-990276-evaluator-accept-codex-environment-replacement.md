# Accept codex environment replacement

- Local timestamp / acting role / milestone: 2026-11-04T15:11:39.990276-05:00 / evaluator / codex-startup-environment
- Requested scope: Adapt actual Codex MCP process environment code while preserving protected Fetch account/helper paths and service-owned credentials.
- Starting repository state: Concurrent source changes from other tasks remain dirty; source review/fixtures prepared.
- Work performed / changed files: Independent read-only review compared actual pinned upstream source, constants, transport behavior, licenses and independent test/build evidence. Attribution-path finding corrected. No remaining scoped source finding. Explicit ACCEPT for bounded environment replacement.
- Decisions: Pinned commit75424f87ca2b1c482e0509a5b1ceb7459587f396; retain actual upstream inheritance/CA/blocked-name rules with Fetch explicit private overrides. No new provider-specific startup/login behavior.
- Checks and evidence: cargo test --locked --test program_paths --test environment_paths --test service_startup_logs --test standard_authentication --test isolated_native_mcp:24/24 pass. Temporary rustc helper harness8/8 pass and removed. cargo build --locked --bin fetch PASS. Real disposable startup/setup peers ran under verified deny-default confinement; account/cwd/args/helper assertions pass. See ../codex-startup-environment/evidence.md.
- Resulting state / acceptance: ACCEPT; limitations include blocked full library tests, no actual new venv-child run (existing unit coverage only) and no real provider authentication.
- Remaining work and handoff: Once concurrent terminal test wiring settles, run focused library helper/native-storage/program-path filters. Real provider/app acceptance is separate; failed explicit Restart→Authenticate observation is unchanged. No commit/push requested/performed.
