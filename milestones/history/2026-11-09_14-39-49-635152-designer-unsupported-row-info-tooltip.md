# Unsupported Figma Remote row and information tooltip

- Local timestamp / acting role / milestone: 2026-11-09T14:39:49.635152-05:00 / Designer, direct reviewer / neumorphic-figma.
- Requested scope: Gray unsupported Figma Remote row and put approval text in information i.
- Starting repository state: Prior requested Figma edits present; unsupported rows used disabled checkboxes but dark labels and separate explanation lines.
- Work performed / changed files: Gray label and checkbox faces in120:1418/1449/1482. Created shared information set234:2431 and row wrappers234:2433/2438/2443; removed separate120:1419/1450/1483. Hover retains the exact approval explanation. Updated design/shared-component/coverage docs, notes/context and milestone journal.
- Decisions: Preserve unsupported/disabled selection rules and exact text; only location and appearance changed.
- Checks and evidence: Structural returns for three states. Results screenshot inspected with gray row and inline i; tooltip screenshot inspected with exact complete explanation. Hover CHANGE_TO link created. Checkbox vector fills corrected after inspection. First stale source-ID attempt rolled back; retry used current source IDs. No live prototype pointer test, native runtime or independent review claimed.
- Resulting state / acceptance: Requested Figma changes complete and visually checked.
- Remaining work and handoff: Await next specific edit; source/native changes remain separate.
