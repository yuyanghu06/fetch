# Simplify disclosure and add server actions

- Local timestamp / acting role / milestone: 2026-11-07T19:31:28.739542-05:00 / designer / neumorphic-figma
- Requested scope: Latest user feedback on server card disclosure, restart and authentication actions.
- Starting repository state: Compact18px header names and colored dots implemented; user disliked the remaining circular disclosure and requested two new icon actions.
- Work performed / changed files: Removed disclosure circles/shadows;14px muted glyph with28px transparent target. Added black/white solid key glyph, Raised/Disabled Authentication icon variants, and shared Server actions132:1678. Updated8card masters/13native headers; order authentication/restart/removal. Corrected instance-swap defaults using explicit per-control glyph assignment. Preserved disabled close variants; maintained saturated red per prior user preference. Added actions to6review rows.120local mains/14sets; root screenshot and no-overlap checks pass. Root serializes docs/notes/journal/evidence updates.
- Decisions: New design only; no Rust/native/API wiring or server/authentication execution. Keep starting/restarting yellow; preserve action availability. Colors do not imply a disabled action became enabled.
- Checks and evidence: Figma screenshots server-header-review.png, server-headers-app.png, server-header-starting.png under milestones/neumorphic-figma/evidence/. All13action groups fit with910pxidentity area and no overlap. Final Evaluator ACCEPT.
- Resulting state / acceptance: Requested Figma header slice complete.
- Remaining work and handoff: User design review and later selected native implementation, including full-header hit target/hover status text and actual authentication/restart controls.
