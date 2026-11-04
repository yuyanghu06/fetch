# Reproduce neighboring sources in selected Codex discovery

- Local timestamp / acting role / milestone: 2026-11-04T17:41:19.076794-05:00 / independent Tester, recorded by Orchestrator / selected-discovery-scope
- Requested scope: Prove the selected-folder bug through the product HTTP discovery route with disposable configurations.
- Starting repository state: Discovery implementation unchanged; Planner accepted; unrelated dirty files preserved.
- Work performed / changed files: tests/backend.rs adjusts existing home/Codex expectations, keeps partial-failure coverage on the home root, and checks installed metadata at the sole Codex group.
- Decisions: Reuse three existing realistic fixtures; no new test infrastructure or speculative cases.
- Checks and evidence: cargo test --locked --test backend discovery_accepts_home_or_codex_folder_and_returns_actual_file_paths -- --exact FAILS as expected on the original source: selected .codex returns 3 groups, expected 1. Home-root iteration passes. Orchestrator inspected test diff.
- Resulting state / acceptance: Original bug independently reproduced; source correction still required.
- Remaining work and handoff: Implement bounded source change, rerun focused discovery checks, build native binary and obtain independent evaluation. No personal configurations, live MCPs, accounts or app launches used.
