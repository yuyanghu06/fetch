# Verify recessed tools scrolling and read-only details

- Local timestamp / acting role / milestone: 2026-11-08T19:11:34.697766-05:00 / independent Tester, serialized by Orchestrator / recessed-tools-demo.
- Requested scope: Verify final three-column populated tools demo, real nested scrolling, recessed Installed servers, enlarged window, read-only raised details and pressed navbar without glow.
- Starting repository state: Scoped Figma implementation complete; only project docs changed locally.
- Work performed / changed files: Independent Tester inspected actual Figma desktop preview and MCP data. No design or source writes.
- Decisions: Use prototype scaling=scale-down / Fit width and height to fit the enlarged artifact on the display. Keep original page scrolling and no Fetch-runtime claims.
- Checks and evidence: Actual demo177:3654 wheel gesture inside tools reaches entire second row (create_directory/move_file/search_files) and24px bottom gap while nav/Installed heading/footer/Search remain stationary. All3columns shown. Full outer window fits at scale-down. Actual Install78:1472 -> Connect78:1593 clicks show correct blue inset selected tab; all3 nav samples use bold white text/two inner shadows only. Twelve read-only values are correct, warmwhite#F7F6F1/raised/no actions; both sections#EAE6DF/two inner shadows. Original read_file JSON retained. Inline CUA screenshots and MCP readbacks are evidence; no local image files saved.
- Resulting state / acceptance: Independent Tester PASS. Final independent evaluation pending.
- Remaining work and handoff: Final evaluation, then deliver direct scroll-demo prototype link. No Rust/native behavior verification claimed.
