# Project session reads

`mod.rs` exposes only list/read operations for the verified connection's project.
Both require the separately approved history permission. Path/project selectors
are rejected; the shared project_sessions service resolves the saved folder and
removes source paths from metadata. Raw log text and chunk continuation remain.
Instruction files and subagent definitions remain native Workspace operations.
