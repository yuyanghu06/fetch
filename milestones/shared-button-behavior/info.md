# Shared button behavior

Goal: make repeated native controls use the same components and make Install MCPs refresh behave like Servers refresh.

Scope: shared refresh presentation/click blocking, shared request/minimum-feedback state, and shared text/primary button styling across native pages. Existing APIs, labels and operation ownership remain.

Completion criteria: actual click/render checks prove pending spin without hover, duplicate-click blocking, immediate results, minimum feedback and retry after success/error; existing native page/control checks and signed build pass.

Status: implementation and direct checks complete; live observation awaits user Quit/reopen.

Refresh controls on Servers and Install MCPs now use the same shared rendering and running-action guard: blue hover glow, clockwise hover rotation, continued rotation without hover while running, and no second click during activity. Server-list and discovery reads share RefreshFeedback without delaying returned results; the Servers minimum is 1.5 seconds and discovery minimum is three seconds. Server restart continues spinning through its real pending operation and now also uses the component-owned click guard. Text actions across pages, native connector setup/approval, credentials and Terminal reuse text_button; blue Install/Save actions reuse primary_button. Existing specialized copy/tab/disclosure controls retain their common implementations and purposes.

78 UI checks pass (10.59s), including the new real-widget discovery success/failure/retry case and existing hover, server reload, restart, copy, Settings, connector-popup and terminal checks. Inspected Install idle/feedback and minimum-size Connect PNGs. Signed development build-only passes (20.85s) and the launcher’s strict signature verification passes. git diff --check passes. The first wider run found a stale Connect assertion expecting pre-existing HTTP commands; it now checks the current stdio connector command already implemented elsewhere in the working tree. Existing compiler/deprecation warnings remain.

Checks/review were performed directly and sequentially, without subagents or independent review, as requested. No Fetch relaunch, MCP/provider call, client-config edit, credential/trust/signing-selection change, Figma edit, commit/push or release. Running Fetch retains its earlier executable; explicit Quit/reopen is needed for live observation.

[Evidence](evidence/info.md).

## November 12 follow-up

Installed servers Refresh now uses a 1.5-second minimum indicator and continues spinning if the read takes longer. Install MCPs Refresh is now in the path-input row, immediately left of Choose file or folder…. Its three-second minimum is unchanged. Both use the same shared feedback implementation; the caller supplies the duration. Returned data still applies immediately.

Four focused existing UI checks pass: manual refresh pending spin, instant reload/minimum feedback, discovery click/spin/retry, and normal/minimum-window screens. Inspected the 874×630 filled-path and 760×520 Install screenshots. Signed development build-only passes (15.03s), including strict signature verification; git diff --check passes. No new tests were added for this small adjustment.

Direct work/review without subagents. No Fetch relaunch or live MCP/provider calls; running Fetch needs explicit Quit/reopen to show the changes. Unrelated edits, credentials, personal settings and services were preserved.
