# Preserve partial refactor and pause for full plan

- Local timestamp / acting role / milestone: 2026-11-03T20:03:53.183471-05:00 / Orchestrator serializing coder/tester reports / native-server-storage.
- Requested scope: approved Keychain option, followed by explicit pause to plan all changes/routes and discuss MCP-specific code before writing.
- Starting repository state:515c7b1 with inventory/proposal documentation.
- Work performed / changed files: partial Manager rewrite; API credentials removal/open-folder registration; execution-only MCP schema; native capability/UI credential removals; remote_oauth Keychain boundary/build registration; two integration fixture updates. No credentials module or OS credential changes. Current docs/journal now record pause and route plan.
- Decisions: direct Keychain remote-client persistence selected; no additional per-MCP mechanisms selected. All source writers interrupted and told not to edit/build/test.
- Checks and evidence: read-only git status/diff/rg and specialist reports; no build/test/runtime checks. Manager references missing modules/functions and old helper/deletion source remains, so compile is not established and source is explicitly incomplete.
- Resulting state / acceptance: paused for full plan; partial source retained, no acceptance claimed.
- Remaining work and handoff: discuss native folder/helper/env-only setup and existing Railway extra CLI; user reviews route/non-route plan before source resumes. Later independent confined verification/evaluation.
