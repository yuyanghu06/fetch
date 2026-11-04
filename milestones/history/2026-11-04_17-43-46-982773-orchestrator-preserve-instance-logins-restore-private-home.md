# Preserve instance logins and restore private HOME

- Local timestamp / acting role / milestone: 2026-11-04T17:43:46.982773-05:00 / orchestrator with Coder report / normal-system-home
- Requested scope: User clarifies locally installed Railway CLI login must belong to that Fetch instance, not the global account; asks for basic Keychain explanation before proceeding.
- Starting repository state: Root mistakenly interpreted package-preservation reply as normal-HOME approval and briefly assigned that source change.
- Work performed / changed files: Coder briefly removed HOME override/renamed local private-folder helper in native_state.rs, then restored ONLY that pass after clarification. Root checked explicit private HOME assignment and home helper are restored. No build, launch, provider login, Keychain reads or credential changes occurred.
- Decisions: Private instance login remains required. Normal HOME inheritance is withdrawn as a solution because it can reuse global HOME-based credentials. Package installation location does not determine credential storage. Shared service Keychain namespaces also require care. All private package/helper PATH changes from earlier remain.
- Checks and evidence: Coder restoration report; root source check confirms private HOME. Tester instructed to stop dependent expectation changes/runs and restore only their pending HOME expectations. No acceptance claimed for an alternative fix.
- Resulting state / acceptance: Original private-home behavior restored; Keychain problem unresolved. Research-only alternatives in progress; no new storage policy selected.
- Remaining work and handoff: Explain OS Keychain vs app entries and storage paths plainly. Research supported native per-instance storage or Keychain selection before another proposal; do not resume HOME change.
