# Integrate development state

- Local timestamp / acting role / milestone: 2026-10-28_17-17-15-069405 America/New_York / Orchestrator / development-state.
- Requested scope: portable development-state separation and shared Fetch/fresh Codex test flow.
- Starting state: cloned main 91d491f plus first-session planning docs.
- Work: inspected actual three-source-file diff and independent regression test source; integrated source/docs after independent evaluator acceptance and requested wording fixes. Documented user-home-relative paths, optimized-build override, explicit overrides, no migration, global discovery, test-only CODEX_HOME, distinct client grants/shared provider sessions, empty test OAuth store and TLS prerequisites.
- Changed files: source in application.rs/main.rs/manager/mod.rs; tests/environment_paths.rs; README, AGENTS, PROJECT_CONTEXT, NOTES; docs/testing guides and architecture decisions; affected source/test folder guides and milestone journal.
- Evidence: source reads and Git diff/status only. Four regression tests authored but not run. No agents ran compilation, tests, app/services, installations, connections, credentials, personal config writes, certificate trust or packaging.
- Decisions: source changes implement user-requested separation; client config/account/TLS setup and runtime checks remain pending. No release or auto-migration.
- Result: source/documentation integrated locally; uncommitted. No current runtime acceptance.
- Handoff: docs/testing/DEVELOPMENT.md contains launch paths and two-client workflow. Rebuild/quit old binary before using new defaults. Preserve all earlier pending acceptance.
