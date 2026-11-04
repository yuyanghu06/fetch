# Automatic discovery independent fixture checks

- Local timestamp / acting role / milestone: 2026-11-03T19:26:54.226235-05:00 / independent Tester / automatic-config-discovery.
- Requested scope: test automatic defaults, optional file/folder discovery, read-only behavior and UI request state without personal files or connections.
- Starting repository state: Coder backend and Orchestrator UI edits ready; tester owns tests/backend.rs and src/ui/auth_tests.rs.
- Work performed / changed files: seven backend cases and three Desktop message cases; fixture initialization adds new UI state fields. Checks cover default paths, home/.codex inputs, explicit-file isolation, malformed sibling sources, injected CODEX_HOME, empty/missing/symlinked folders (including trailing slash), authorization, no recursion/secret output/source mutation and default/custom UI flow with busy/picker/install guards.
- Decisions: only disposable files and in-process requests; dropped Desktop Tasks are never executed. Native appearance remains separate.
- Checks and evidence: DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test --locked --test backend --test import_compatibility passes 23 backend and 7 import cases. cargo test --locked --test backend --test import_compatibility --lib ui::auth_tests passes 17 UI cases (filter selects zero integration cases in that command). Total 47 relevant distinct cases pass, 10 new. Focused 9 discovery and 8 Desktop cases also passed. cargo build --locked --bin fetch --message-format short succeeds exit 0 (cached, 0.15s); git diff --check passes. Existing native bridge deprecations and unused MCP error field warnings remain.
- Resulting state / acceptance: independent tests and native build pass; no apps/windows, Task network execution, Keychain, provider or real MCP connection performed.
- Remaining work and handoff: final independent evaluation; actual native picker/appearance and Figma update remain pending as documented.
