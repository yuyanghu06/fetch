# config picker source acceptance

- Local timestamp / acting role / milestone: 2026-10-28_17-31-23-020388 America/New_York / evaluator / config-file-picker.
- Requested scope: add file selector beside optional config path so users can choose their MCP configuration file.
- Starting state: main91d491f plus existing uncommitted development-default source/docs and prior binary evidence; preserve all unrelated changes.
- Work / evidence: Independently ACCEPTED source with no requested fixes. Reviewed native panel settings/main-thread async behavior, FFI callback lifetime, duplicate/stale handling and absence of implicit reads/discovery/install/auth/config changes. Did not run checks or UI. Root supplied successful compilation evidence.
- Changed files: writers owned the three UI/native files above; Orchestrator owns affected folder guides, development guide, context/notes and serialized journal.
- Decisions: standard native Open sheet, one file/no directories/no extension restriction; current-user home/hidden files; selection fills path only; Discover servers remains explicit.
- Result: bounded source acceptance; native interaction not demonstrated.
- Handoff: Root binary compilation succeeded; relaunch updated dev build for manual picker checks. No credentials, personal configs, managed connections, packaging or release were touched.
