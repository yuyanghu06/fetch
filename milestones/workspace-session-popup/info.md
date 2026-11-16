# Workspace presentation and folder-picker response

Goal: remove redundant instruction selection, show session history as conversation turns in a popup, gray the session list while loading, and finish requested heading/menu/picker fixes.

Scope: existing native UI, shared surfaces, read-only JSON presentation and AppKit panel preparation. Existing Workspace API routes own discovery/file/log reads. No new route, permission, dependency, personal-state change or app launch.

Completion: requested source changes, focused fixture checks, inspected normal/minimum renders and signed development build pass. Live dialog response and real personal session rendering remain user observation. Same-agent source review; no independent review for this direct UI pass.

Evidence in evidence/: workspace-tests.txt and workspace-visual-tests.txt (five cases), session-parser-tests.txt (two), project-controls-tests.txt (one), server-menu-capture-retest.txt (one), native-picker.txt (hidden fixture), signed-build.txt, and PNG renders. server-menu-tests.txt preserves an initial fixture failure because pick-list labels do not expose text operations; the test now clicks its actual screen position. Capture was subsequently corrected to draw the menu in a separate renderer layer; product menu code did not change for this correction.

Read research.md for source formats and compared implementations. Displayed messages are a chronological projection of loaded records, not reconstruction of compacted/forked session branches. Images are attachment labels; raw log bytes and files are unchanged.
