# Keep Installed servers fixed and scroll its cards

- Local timestamp / acting role / milestone: 2026-11-09T17:47:33.112036-05:00 / Primary agent, sequential plan/implementation / fixed-installed-server-list.
- Requested scope: keep the pictured Installed servers panel fixed and scroll server cards inside it.
- Starting repository state: prior navbar and ready-tooltip fixes plus existing Figma records uncommitted; preserved.
- Work performed / changed files: src/ui/mod.rs removes the outer Servers-page scroll, fits the page/section into available window height, leaves title/Refresh outside a dedicated fetch-installed-servers card scroller, and preserves heading/first-card spacing. src/ui/neumorphic_tests.rs adds one actual-widget regression with eight disposable servers at normal/minimum sizes, expansion, wheel scrolling, fixed header/bounds checks and scrolled-card clicking.
- Decisions: fixed within the window, resizing with available space; no arbitrary height added. Navigation and title remain visible. Existing nested panels and API/data behavior remain. No Figma write or speculative feature.
- Checks and evidence: implementation inspected; focused widget suite in progress. No passing result claimed yet.
- Resulting state / acceptance: ready for interaction/render checks and signed build.
- Remaining work and handoff: inspect results, resolve observed findings, visually check captures, build/sign and record direct review. Existing running app preserved.
