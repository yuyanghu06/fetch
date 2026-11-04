# Complete codex environment replacement

- Local timestamp / acting role / milestone: 2026-11-04T15:11:39.990277-05:00 / orchestrator / codex-startup-environment
- Requested scope: Adapt actual Codex MCP process environment code while preserving protected Fetch account/helper paths and service-owned credentials.
- Starting repository state: Concurrent source changes from other tasks remain dirty; source review/fixtures prepared.
- Work performed / changed files: Inspected actual helper/transport/fixtures, retained exact upstream Apache LICENSE/NOTICE and corrected source attribution, integrated independent reports and refreshed milestone/current architecture/folder guides. Preserved concurrent terminal/auth/logging changes and personal state.
- Decisions: Pinned commit75424f87ca2b1c482e0509a5b1ceb7459587f396; retain actual upstream inheritance/CA/blocked-name rules with Fetch explicit private overrides. No new provider-specific startup/login behavior.
- Checks and evidence: cargo test --locked --test program_paths --test environment_paths --test service_startup_logs --test standard_authentication --test isolated_native_mcp:24/24 pass. Temporary rustc helper harness8/8 pass and removed. cargo build --locked --bin fetch PASS. Real disposable startup/setup peers ran under verified deny-default confinement; account/cwd/args/helper assertions pass. See ../codex-startup-environment/evidence.md.
- Resulting state / acceptance: Scoped replacement complete. Existing private directories/helper PATH and service credential ownership remain. No Codex OAuth/store import, public API/schema/dependency addition, app launch, personal config edit or release work.
- Remaining work and handoff: Once concurrent terminal test wiring settles, run focused library helper/native-storage/program-path filters. Real provider/app acceptance is separate; failed explicit Restart→Authenticate observation is unchanged. No commit/push requested/performed.
