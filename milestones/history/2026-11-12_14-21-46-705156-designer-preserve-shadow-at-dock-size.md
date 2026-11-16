# Keep the shadow visible at Dock size

- Local timestamp / acting role / milestone: 2026-11-12T14:21:46.705156-05:00 / Designer-Orchestrator direct diagnosis/fix/check / app-icon.
- Requested scope: investigate reported loss of shadow in actual macOS icon versus Figma reference.
- Starting repository state: export308:799/current app running PID48643; unrelated source/docs changes preserved.
- Work performed / changed files: ran existing native fixture at96px via disposable temporary source/executable; saved before/after PNGs and logs. Kept large mockup unchanged, made export310:799 with stronger wider bevel/outer shadow. Regenerated canonical packaging PNG/ICNS72DPI and signed build; refreshed current docs/context/notes/guides/tracking.
- Decisions: evidence indicates loss of visibility at reduced scale, not missing shadow data. Do not claim real Dock renderer was independently read. Export-specific effects preserve large Figma design/white/dog/size/corners; source267:800 unchanged, old308 archived.
- Checks and evidence: before96px mean0.156331/255,max6; after96px mean0.162326/255,max6, both actual native assignment and queued-copy lifetime pass/no windows. Before/after PNGs visually inspected. Signed build-only26.85s/strict verification and diffcheck pass. Evidence dock-shadow-*,dock-bevel-signed-build.txt. Temporary fixture state cleaned.
- Resulting state / acceptance: requested observed issue addressed with readable small-size export; direct checks, no agents/new source behavior. Existing app kept running; no services/config/credentials/release/package changes.
- Remaining work and handoff: user explicitly Quit/reopen to view updated actual Dock. Use export310:799 overrides for future canonical exports.
