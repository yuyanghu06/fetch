# Figma Remote unsupported import — Orchestrator

- Local timestamp / acting role / milestone: 2026-11-04T18:07:32.270262-05:00 / Orchestrator / figma-remote-unsupported
- Requested scope: Mark Figma Remote unsupported in Fetch’s import/install screen.
- Starting repository state: Implemented narrow classifier/UI change, two new focused tests,19 passing tests and native build.
- Work performed / changed files: Inspected source and actual19test/native build evidence, accepted independent review, synchronized selected support status and completion records. Preserved unrelated changes. Source: src/discovery/mod.rs, src/ui/mod.rs; tests: tests/import_compatibility.rs, tests/direct_install.rs. Orchestrator owns project/architecture/UI/folder/journal docs.
- Decisions: No new routes/schema or provider workflow; default bridge parser remains unchanged. Remote-only unsupported selection preserves Desktop/saved records.
- Checks and evidence: cargo test --locked --test import_compatibility --test direct_install --message-format short PASS19/19 (2new). cargo build --locked --bin fetch --message-format short PASS5.38s. Scoped diff --check PASS. Actual source/tests and /tmp/fetch-figma-unsupported-tests.log and /tmp/fetch-figma-unsupported-build.log inspected.
- Resulting state / acceptance: ACCEPT; requested change complete in rebuilt source/executable. Native appearance unobserved, no app launch/provider/account/personal state operations.
- Remaining work and handoff: User can explicitly Quit Fetch then run ./scripts/run-development.sh and refresh imports to observe it. Existing installed entries stay unchanged; no approval request, alternate connector or release selected.
