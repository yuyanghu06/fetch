# config picker source

- Local timestamp / acting role / milestone: 2026-10-28_17-31-23-020386 America/New_York / coder / config-file-picker.
- Requested scope: add file selector beside optional config path so users can choose their MCP configuration file.
- Starting state: main91d491f plus existing uncommitted development-default source/docs and prior binary evidence; preserve all unrelated changes.
- Work / evidence: Exclusively edited src/ui/mod.rs, new src/ui/file_picker.rs and src/native_auth/bridge.m. Added Choose file… in path row, async main-queue AppKit sheet/oneshot callback, absolute UTF8 path handling, cancel preservation, duplicate/operation guards and close/Quit cancellation with old-window result rejection. No execution.
- Changed files: writers owned the three UI/native files above; Orchestrator owns affected folder guides, development guide, context/notes and serialized journal.
- Decisions: standard native Open sheet, one file/no directories/no extension restriction; current-user home/hidden files; selection fills path only; Discover servers remains explicit.
- Result: bounded source acceptance; native interaction not demonstrated.
- Handoff: Root binary compilation succeeded; relaunch updated dev build for manual picker checks. No credentials, personal configs, managed connections, packaging or release were touched.
