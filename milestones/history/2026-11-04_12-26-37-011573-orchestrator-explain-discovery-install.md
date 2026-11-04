# Explain discovered server installation

- Local timestamp / acting role / milestone: 2026-11-04T12:26:37.011573-05:00 / Orchestrator / startup-diagnostics
- Requested scope: Check whether discovered servers use install commands or copied original files.
- Starting repository state: Original Google app-value omission confirmed; existing source work remains uncommitted.
- Work performed / changed files: Read-only inspection of src/discovery/mod.rs and src/installation/mod.rs; journal/notes only.
- Decisions: No implementation or credential-copy changes.
- Checks and evidence: Discovery reads definitions only. Selected npm entries run private npm install and save resolved executable; PyPI entries run managed Python/venv plus uv pip install; local scripts remain at their original paths; remote entries save connection definitions. prepare_group invokes normal Connect hook after preparation; saved matching associations can reuse previous records. Source environment/header values omitted during capture. No runtime/install/test commands executed in this pass.
- Resulting state / acceptance: Explanation grounded in current source. Package download does not reproduce separately supplied application credentials.
- Remaining work and handoff: Importing application configuration remains a separate policy choice; no personal settings changed.
