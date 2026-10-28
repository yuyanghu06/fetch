# config picker plan

- Local timestamp / acting role / milestone: 2026-10-28_17-31-23-020385 America/New_York / planner / config-file-picker.
- Requested scope: add file selector beside optional config path so users can choose their MCP configuration file.
- Starting state: main91d491f plus existing uncommitted development-default source/docs and prior binary evidence; preserve all unrelated changes.
- Work / evidence: Read UI/client/native bridge/build. Planned native asynchronous one-file picker, hidden folders visible, existing editable field, no API change or automatic discovery/install. Proposed ownership/lifecycle checks. Read-only, no execution.
- Changed files: writers owned the three UI/native files above; Orchestrator owns affected folder guides, development guide, context/notes and serialized journal.
- Decisions: standard native Open sheet, one file/no directories/no extension restriction; current-user home/hidden files; selection fills path only; Discover servers remains explicit.
- Result: bounded source acceptance; native interaction not demonstrated.
- Handoff: Root binary compilation succeeded; relaunch updated dev build for manual picker checks. No credentials, personal configs, managed connections, packaging or release were touched.
