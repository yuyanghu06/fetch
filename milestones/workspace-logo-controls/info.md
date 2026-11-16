# Workspace compact controls and app logos

Goal: remove redundant Global/Open folder and per-app text buttons, move content up, and use one right-aligned pair of circular app choices for all three sections.

Scope: native Workspace layout/presentation and instruction filtering through the existing selected-app state. Reuse existing app actions, APIs, logos and common control effects. No backend/persistence/permission change.

Completion criteria: actual logo clicks drive matching Instructions/Subagents/Sessions; removed controls remain absent; normal/minimum layouts fit; raised/inset choices are visible; focused checks and signed build pass.

Status: implemented and directly verified; live user observation remains pending.

Workspace removes the Global/Open folder button row and the separate Claude Code/Codex text-button row. The scope caption moves into the heading, allowing Instructions/Subagents/Sessions and their content to move up. The tabs row now has two circular app-logo choices on its right, each40px to match tab height. The shared app_logo_button uses existing logos and shared button/surface rendering: unselected raised, selected/pressed inset with the existing recessed fill. One workspace.app selection controls Instructions (CLAUDE.md versus AGENTS.md), Subagents and Sessions; existing session API app filtering is reused. Folder picker and typed-path Enter/global-empty behavior remain through their existing handlers.

Four Workspace widget checks pass (3.67s), including real logo clicks and correct app-specific instructions/subagents/sessions. After strengthening selected-face contrast, the same complete selection case passes again (2.41s). Both selected states and normal/minimum screenshots were inspected. Final signed development build-only passes (16.05s), including strict signature verification; git diff --check passes. An initial new fixture case needed relayout after injected API replies; its original failing log is preserved and only the fixture was corrected.

Direct UI work without subagents or independent review for this pass. Fixture API tasks are dropped: no personal files, native picker, clipboard, model process or live account/provider/MCP action is executed. No API/auth/permission/storage behavior changed, no Figma/commit/push/release, and Fetch was not relaunched. User must explicitly Quit/reopen to observe the rebuilt UI. Concurrent project-scoped/Workspace backend work is preserved.

[Evidence](evidence/info.md).
