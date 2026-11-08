# Refine shared neumorphic controls

- Local timestamp / acting role / milestone: 2026-11-07T18:25:29.915213-05:00 / designer / neumorphic-figma
- Requested scope: Apply live icon, color, depth, Copy, text and spacing feedback.
- Starting repository state: Earlier neumorphic Figma baseline and icon pass complete; existing unrelated changes preserved.
- Work performed / changed files: Updated Figma pages75:2251/75:2718/75:2719: gray right-aligned gear, outlined when unselected/filled when selected; blue Refresh; retained red#C64545 with centered18.2px white vector× and convex depth; white20px Copy glyph on Console and matching blue glyph on light command blocks; all accent button text white Bold; removed checkbox outlines/restored white checks; fields#EEEFEF; Settings32px bottom padding and scrolled framing. Original current Figma server headers changed only to match the explicit code fix.
- Decisions: Keep this pass bounded to the user-selected presentation changes; no API or authentication behavior change.
- Checks and evidence: Representative screenshots in /tmp/fetch-neumorphic/refined-*.png; independent visual Evaluator ACCEPT for visible controls. Separate unscrolled Settings screenshot confirms filled selected gear. Convex blue depth restored after root initially flattened it too much.
- Resulting state / acceptance: Style changes applied.
- Remaining work and handoff: Latest installed-checkbox gray request and new Install popup UX tracked in next pass; full Rust neumorphic port remains unimplemented.
