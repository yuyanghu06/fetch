# Match the full raised frame edge

- Local timestamp / acting role / milestone: 2026-11-12T12:59:30.563617-05:00 / designer / logo-mockups.
- Requested scope: user corrected the corner-only change; match actual soft app frame boundary.
- Starting repository state: prior32px clipping-only edit and existing unrelated changes, preserved.
- Work performed / changed files: read app surface97:2015 and shared Raised panel style; applied existing effect style to logo267:800, bound its four corners to neo/radius/32. Updated docs/FIGMA, logo folder/milestone guides, snapshot and to-do.
- Decisions: reuse exact app effect style S:fcd24633bfabe0ac8f92270481c443d43380c1f3, with two inner and three outer shadows. These effects belong to the frame, not the dog.
- Checks and evidence: effect style/radius bindings read back; original imageHash88141f7dfaeb37d8692624229828c942ee1f3d4a retained. Corrected composition screenshot inspected directly.
- Resulting state / acceptance: requested full raised frame treatment applied. Previous zero-effects state superseded by this correction; history preserved. No app code/build/test/launch or subagents.
- Remaining work and handoff: original PNG unchanged; Figma owns editable frame effects. Existing unfinished product tasks preserved.
