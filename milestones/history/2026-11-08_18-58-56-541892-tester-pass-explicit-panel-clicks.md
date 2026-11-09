# Verify explicit server panel clicks

- Local timestamp / acting role / milestone: 2026-11-08T18:58:56.541892-05:00 / independent Tester, journal serialized by Orchestrator / explicit-server-panels.
- Requested scope: Verify real Figma clicks for explicit panel opens, pressed selected controls without glow, redundant restart removal and terminal alignment.
- Starting repository state: Figma implementation complete; root had updated only project documentation locally.
- Work performed / changed files: Independent Tester used actual Figma desktop prototype and read-only MCP inspection. Found Selected labels showing Button because cloned label172:1911 lost its property reference. Root restored componentPropertyReferences.characters to Label#21:2. Tester repeated the affected click flow; no other design/source changes during testing.
- Decisions: Preserve original scrolling. Settings selected appearance is represented in the shared four-control specimen; the existing Settings navigation remains illustrative and shows selected server search. Do not claim executable Shift-click in Figma.
- Checks and evidence: Actual closed170:2698 -> Inspect tools78:1438 (Console hidden) -> close -> Console78:1705 -> close -> Terminal78:1752 PASS after label fix. Correct selected labels and pressed appearance; no selected glow. Four-control review172:3525 has correct labels/two INNER_SHADOW effects each. Both terminal masters read1072px/FILL, matching parents. Redundant Recovery restart controls absent. Inline CUA screenshots and MCP readbacks are evidence; no local image files created.
- Resulting state / acceptance: Independent Tester PASS; final independent evaluation pending.
- Remaining work and handoff: Final evaluation. Shift-click force remains selected later-application behavior, not simulated by prototype. No Fetch runtime/build/source change.
