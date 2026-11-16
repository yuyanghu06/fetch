# Add subtle dog shadow

- Local timestamp / acting role / milestone: 2026-11-12T13:32:47.401064-05:00 / designer / logo-mockups.
- Requested scope: add a little drop shadow to the dog.
- Starting repository state: midpoint-blue tongue dog v6; unrelated edits preserved.
- Work performed / changed files: one native DROP_SHADOW on group285:801,14px radius,0/6offset,rgba(.35,.43,.48,.16),spread0. Save docs/logo-mockups/fetch-dog-midblue-shadow-v7.png and update docs/FIGMA, logo/milestone guides, snapshot/to-do.
- Decisions: shadow only on masked dog silhouette, keep previous PNG/background unchanged; no regeneration.
- Checks and evidence: Figma effect readback and composition screenshot inspected; image793×793 at absolute237,235 retained. Isolated PNG821×821 includes extra shadow bounds.
- Resulting state / acceptance: requested shadow applied; subjective acceptance remains with user. Direct check, no subagents/app code/build/test/launch.
- Remaining work and handoff: v6 remains unshadowed; v7 includes shadow. Current Figma effect can be adjusted separately.
