# Plan the precise Figma appearance correction

- Local timestamp / acting role / milestone: 2026-11-09T14:59:35.744214-05:00 / Planner, Researcher and Orchestrator / neumorphic-fidelity.
- Requested scope: user says every surface looks too glossy and requires pixel-perfect Figma appearance.
- Starting repository state: previous native port and its documentation remain uncommitted; functional checks passed but appearance used gradients and simplified shadows.
- Work performed / changed files: read current source/context/notes/architecture and shared controls. Reused official Figma task session; read fresh Button/Navigation/server/app high-fidelity contexts and native-size references. Independent planner identified exact typography/gap/radius/number-field mismatches. Independent Researcher compared licensed maintained Iced/resvg with custom GPU/quad alternatives; selected existing SVG filter rendering. Added scoped milestone, test module declaration and static Inter Medium500 font.
- Decisions: flat design fills plus exact complete inner/outer shadow layers using existing renderer; no new dependencies or product/API changes. Match components before accepting screens; preserve prior actual interactions and minimum-window readability.
- Checks and evidence: Figma Button reference524x208 and exact child bounds recorded; fullsize app/server references downloaded. resvg filter primitives and Iced SVG caching verified from versioned primary source. Prior acceptance is superseded for visual fidelity by explicit user rejection.
- Resulting state / acceptance: bounded correction implementation underway; visual acceptance pending real comparisons.
- Remaining work and handoff: finish shared rendering and geometry, equal-size captures/comparisons, independent tests/evaluation, build and record honest residual differences. Dedicated development signing setup remains absent; no launch planned.
