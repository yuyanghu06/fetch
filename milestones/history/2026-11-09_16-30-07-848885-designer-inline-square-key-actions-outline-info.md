# Inline square credential actions and outlined information icon

- Local timestamp / acting role / milestone: 2026-11-09T16:30:07.848885-05:00 / Designer and direct reviewer / neumorphic-figma.
- Requested scope: Move Save/Remove inline as square blocks with white Save SVG/existing white x; gray Figma Remote and make information i a gray circular outline.
- Starting repository state: Prior user-directed Figma work and documentation present; source unchanged this session.
- Work performed / changed files: Shared credential action set246:2464, four variants, applied to75:2350/2358/2366/2374. New inline rows retain existing field/eye and original availability/opacity; old text buttons removed. Shared information circle/text234:2422/2423/2427/2428 restyled. Actual app discovery label/checkbox overrides fixed in121:2731/2801/2872. Updated notes/context, Figma/shared-component/state docs and milestone journal.
- Decisions: Small44×44 square actions; existing white x reused; imported Simple Design System Save vector reused and made white. Disabled opacity0.58 preserved. No behavior/API changes.
- Checks and evidence: Shared filled editor and actual filled Settings row screenshots inspected. Initial dialog screenshot exposed black literal fallback in variable-bound instance overrides; corrected paint literals to resolved colors and inspected updated dialog screenshot. All three unsupported labels read back with gray RGB values. Information outline visible; tooltip message unchanged. No independent review, native tests or runtime claim.
- Resulting state / acceptance: Requested Figma changes complete and visually checked.
- Remaining work and handoff: Await next exact edit; native port and noted spinning restart button remain separate.
