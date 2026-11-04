# Integrated discovery change accepted

- Local timestamp / acting role / milestone: 2026-11-03T19:31:19.393149-05:00 / independent Evaluator and Orchestrator / automatic-config-discovery.
- Requested scope: existing discovery route accepts a .codex-folder path and finds configuration files itself; preserve explicit file overrides and newer main changes.
- Starting repository state: resolved merge, independent integrated checks pass; two tester regression additions require final staging.
- Work performed / changed files: independent review against both merge parents and the final tests. Orchestrator finalizes/stages journal and rolling status.
- Decisions: Evaluator ACCEPT with no required fixes. Preserve direct-install normalization/capture, installed flags/runtime handler, disabled installed selections, deletion and auth source. Ordinary users need not supply individual config filenames.
- Checks and evidence: 64 relevant tests pass (24 backend, 7 import, 8 direct, 18 UI, 7 discovery), 12 new. Final native build exit 0. Scoped whitespace check against origin/main excludes imported immutable evidence logs; no conflict markers/unresolved entries remain. Three blank-line-at-EOF warnings belong to incoming deletion evidence and are preserved.
- Resulting state / acceptance: merged source/fixture scope complete and accepted, ready for normal merge commit/push.
- Remaining work and handoff: delivery record after push; actual picker/window and refreshed Figma specimens remain pending, alongside unchanged broader protected-runtime work.
