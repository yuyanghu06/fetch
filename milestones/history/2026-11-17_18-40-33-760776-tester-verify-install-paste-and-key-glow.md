# Verify the install, instruction and auth control changes

- Local timestamp / acting role / milestone: 2026-11-17T18:40:33-05:00 / Tester / Native UI follow-ups
- Requested scope: Independently check the three affected native UI areas.
- Starting repository state: The updated controls and tooltip were available to the existing widget fixtures.
- Work performed / changed files: Updated src/ui/neumorphic_tests.rs and src/ui/workspace_tests.rs with offscreen widget events and pixel checks.
- Decisions: Use simulated installation and clipboard replies. Verify both auth key sizes and the nearby Refresh glow.
- Checks and evidence: Six focused checks passed. Two install checks sampled white and gray boxes and revealed the error on hover. Two editor checks covered Paste and Copy order, draft/Undo and stale replies. Two auth checks verified neutral gray darkening and the blue Refresh tint.
- Resulting state / acceptance: PASS. Rendered captures and source behavior were inspected.
- Remaining work and handoff: Live app, clipboard and real MCP installation were outside these checks.
