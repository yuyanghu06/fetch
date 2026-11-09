# Implement inline server settings and controls

- Local timestamp / acting role / milestone: 2026-11-09T13:36:16.905683-05:00 / designer / inline-server-settings.
- Requested scope: Inline per-server Settings, no navbar Settings button, requested copy removal and two-column numeric controls.
- Starting repository state: Existing local design documentation and journal changes; no application source changes.
- Work performed / changed files: Root created shared Settings card family202:2558 with5 variants and1072x440 scroll panels. Repurposed6 old Settings frame IDs to inline Filesystem/Search states, added Search closed204:4868, removed new-board Settings heading and shared nav gear100:1522, and wired each Settings button to its own server. Removed exact helper sentence. Replaced Fetch call limits heading/two descriptions with side-by-side17pxBold Time until timeout and Max concurrent calls. Shared Number input212:2318 retains120/1 with right up/down arrows. Light Copy115:1371 has no fill/stroke/effect; blue glyph/click area retained.
- Decisions: Figma new-design pages only; preserve original references, source, personal settings and backend behavior. Root serializes shared records.
- Checks and evidence: Root screenshots show correct inline identity, values, no nav gear, no selected glow. Five limit rows each have500px fields/24px gap/44px inputs,17pxBold headings and20px title-input gap; transparent layout wrappers avoid a white rectangle. Source filled/busy/folder states reused. Prototype number arrows are visual affordances only, not numeric runtime behavior. No source writes.
- Resulting state / acceptance: Implementation complete; independent final structure/render verification pending.
- Remaining work and handoff: Final focused checks/evaluation. Do not interfere with active user Figma UI; no Fetch/build/runtime claims.
