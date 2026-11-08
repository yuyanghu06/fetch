# Preserve header action shadows

- Local timestamp / acting role / milestone: 2026-11-07T19:32:19.520137-05:00 / designer-orchestrator / neumorphic-figma
- Requested scope: Retain the previously selected unclipped neumorphic shadows on the new header actions.
- Starting repository state: Header identity/actions accepted; new review-row containers used default clipping.
- Work performed / changed files: Disabled clipping on the shared Server actions component, its instances, server header rows and review rows; updated three final screenshots. No other appearance or behavior changed.
- Decisions: Only actual application/scroll boundaries clip; inner action groups allow shadows to extend.
- Checks and evidence: Targeted post-fix review screenshot confirms soft shadows extend below controls; all previous glyph/data/state checks remain valid. Final git diff --check follows.
- Resulting state / acceptance: Root post-fix visual check passes; prior independent header/action acceptance remains applicable.
- Remaining work and handoff: New design remains Figma-only; later Rust port remains open.
