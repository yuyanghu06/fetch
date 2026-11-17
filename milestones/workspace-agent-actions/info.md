# Workspace model picker and subagent actions

Add and Edit load the app's model list on first open. The native picker displays up to four rows and scrolls through the rest. The model is optional and the app chooses it when the definition leaves it out.

Copy carries the matching Codex registration description beside the original file. Cross-app conversion uses it for a missing description. A Codex read-only setting maps to Claude Plan.

Each subagent card has Copy, Paste and a red X. Paste automatically replaces the selected definition with the first copied entry and creates other eligible entries. Already occupied names are protected for the remaining entries. The X opens a confirmation naming the file. Delete protects the shared Codex configuration.

Seven focused checks, independent review and the signed development build passed. Verification used temporary app files and widget events.

- [Four-row menu](model-menu.png)
- [Scrolled menu](model-menu-scrolled.png)
- [Row Paste result](row-paste.png)
- [Delete confirmation](delete-confirmation.png)
- [Focused checks](focused-tests.txt)

Quit and reopen Fetch to use the updated executable. Copy again to put the registered description in the clipboard.

## Toolbar Paste follow-up

Toolbar Paste accepts a single copied card or a Copy all bundle from the current clipboard. The focused tests confirmed immediate file creation, the original filename even when a card points to a differently named file, and existing filename conflict protection. Review and the signed development build passed. [Focused check](../history/2026-11-17_17-51-23-321603-tester-verify-single-card-toolbar-paste.md)
