# Workspace

An API-backed native page for global/project instruction files, existing Claude
and Codex subagents, converted paste drafts, dynamic app model choices and bounded
read-only session streams. Global shows global definitions and all sessions;
selected folder shows only that project. Native APIs own file handling; UI wraps
them. Files stay in native locations, with no history archive.

Status November12: selected first slice implemented,14 focused checks and signed
development build/strict verification pass; independent Evaluator ACCEPT. See
[evidence](evidence.md) and [approved architecture](../../docs/architecture/workspace.md).
Actual user-file/clipboard/Claude-catalog acceptance awaits explicit Quit/reopen
and manual use. Session MCP is deliberately excluded by user selection until the
separate project connection feature exists. No personal-state changes, release or
automatic app relaunch are part of fixture/build completion.
