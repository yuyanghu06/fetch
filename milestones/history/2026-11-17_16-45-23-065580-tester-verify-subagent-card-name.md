# Verify the displayed name survives conversion

- Local timestamp / acting role / milestone: 2026-11-17T16:45:23-05:00 / Tester / Subagent card name
- Requested scope: Exercise the registered card name, explicit source name and missing description.
- Starting repository state: Clipboard and converter changes ready. The first compile ran out of disk before tests. Generated test files and scratch captures were cleared for the retry.
- Work performed / changed files: Extended the existing Workspace widget and API fixtures. The widget test copies the Codex card for legacy-role.toml, whose displayed label is coder, converts it and verifies the actual temporary Claude file. The HTTP test checks explicit-name precedence and the missing-description result.
- Decisions: Drive the existing file routes with a fixture clipboard and temporary app paths.
- Checks and evidence: Two widget checks passed in 3.12 seconds. One native HTTP check passed in 0.02 seconds. The new Claude definition has name coder and the source bytes still match. [Results](../subagent-card-name/focused-tests.txt)
- Resulting state / acceptance: Focused checks passed.
- Remaining work and handoff: Finish signed build and get a fresh clipboard entry in the running app.
