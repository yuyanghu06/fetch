# Measure real install, echo and deletion

- Local timestamp / acting role / milestone: November 3, 2026 / independent Tester; serialized by Orchestrator / delete-server.
- Requested scope: Install/delete a disposable real MCP and locate latency; user subsequently identified Google.
- Starting repository state: Ignored harness compiled; production source unchanged by investigation. Temporary sandbox and copied public Node runtime prepared.
- Work performed / changed files: Verified OS confinement; corrected two fixture-only setup issues (private directory modes, /dev/null write rule); actual Everything2026.8.31 install/echo/delete; process observation; Rust1000-file flush comparison. Persisted milestones/delete-server/latency-evidence.md.
- Decisions: Continue with Google package preparation/deletion only, without Google process/login/account access. Production optimization remains unapplied.
- Checks and evidence: Confinement positive/negative probes pass. Actual test exit0: install+echo5.593s,13tools3870files611folders; MCP gone within~139ms of delete start; package cleanup20.383s; total delete20.400s; preservation assertions pass. Per-file1000sync_all4559ms (4340ms in flushes) versus one final flush42ms total. Full evidence in latency-evidence.md. No owned MCP processes remain after Everything run.
- Resulting state / acceptance: Cleanup is the demonstrated bottleneck; repeated disk flushes explain measured overhead in comparison. Not native/UI/HTTP/Keychain or Google-account acceptance.
- Remaining work and handoff: Focused Google install/delete comparison, final review, retain evidence then remove all temporary packages/state/processes.
