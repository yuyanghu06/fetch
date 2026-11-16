# Normalize app icon resolution metadata

- Local timestamp / acting role / milestone: 2026-11-12T13:52:34.605227-05:00 / Coder-Orchestrator / app-icon.
- Requested scope: use approved artwork as actual application icon; fix observed native decode issue.
- Starting repository state: first signed build passed; independent native tester found source1024px interpreted as2949points, AppKit snapshot5898px.
- Work performed / changed files: sips set packaging/Fetch.png dpiWidth/dpiHeight72, regenerate Fetch.icns from canonical PNG. packaging/info and snapshot updated. Final signed build logged to evidence/signed-build-final.txt; tester notified for focused rerun.
- Decisions: metadata normalization only, no visual/logo or API change. No live Fetch launch/packaging.
- Checks and evidence: sips reports1024×1024 at72×72DPI. Initial previews matched visually; exact byte equality unsuitable for AppKit resampling.
- Resulting state / acceptance: concrete oversized decode cause fixed in asset metadata; final independent native test/build pending.
- Remaining work and handoff: tester confirms corrected logical size/assignment; evaluator decision; root final integration docs.
