# Implement Console on expansion and combined error output

- Local timestamp / acting role / milestone: 2026-11-11T14:38:35.384633-05:00 / Coder / console-on-expansion.
- Requested scope: automatic Console selection; Fetch error plus captured server output inside Console.
- Starting repository state: independent plan accepted; numerous pre-existing native/design edits preserved.
- Work performed / changed files: src/ui/mod.rs adds console_lines and direct expansion selection, removes standalone card auth/runtime error rows; src/ui/server_terminal.rs adds direct selection and passes server data to shared Console; src/ui/neumorphic_tests.rs adds actual-widget expansion/output/Copy/polling/re-expansion case and updates default panel assertion. Root inspected exact current code and updated src/ui/info.md, docs/SHARED_UI_COMPONENTS.md and docs/UI_STATE_COVERAGE.md.
- Decisions: authentication Flow history remains unchanged; rendering combines it with current Fetch errors. Returned error code/message/extra fields preserved, repeated current error deduplicated, server lines/order retained. Copy uses assembled display lines.
- Checks and evidence: root git diff --check passes for assigned source files. Coder ran no tests/build; independent Tester assigned three focused fixture checks.
- Resulting state / acceptance: implemented, verification pending. No application/provider/MCP launch or personal-state changes.
- Remaining work and handoff: independent focused tests, inspect exported fixture image, signed build and independent Evaluator.
