# Fix embedded terminal drawing

- Local timestamp / acting role / milestone: 2026-11-04T15:24:11.939731-05:00 / Coder and Orchestrator / server-console-terminal.
- Requested scope: user reports actual native Terminal pane is black with no prompt/cursor, while Close terminal is available.
- Starting repository state: initial source corrections handed off; final core tests underway. Earlier appearance remained unobserved until this user screenshot established the blank-pane failure.
- Work performed / changed files: inspected vendored view.rs against Iced0.14 official Canvas implementation. Upstream used absolute layout positions inside a viewport-sized cache. Corrected to widget-local cached geometry sized to full widget, draw-time translation, and bounds/viewport clipping.
- Decisions: reuse existing renderer; narrow embedding correction only. No shell/startup/authentication behavior or new route changed.
- Checks and evidence: source review identifies why nested/scrolled terminal text/cursor could be clipped. Independent Tester is adding an offscreen renderer regression using Iced's existing Headless interface. No passing render/build or rebuilt native observation claimed yet.
- Resulting state / acceptance: source correction ready; final independent check/review pending.
- Remaining work and handoff: core tests, nonzero-offset/clipped/scrolled offscreen display check, native build and final evaluation. User's everyday app remains running/unmodified by agent.
