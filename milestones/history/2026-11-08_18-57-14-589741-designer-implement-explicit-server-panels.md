# Implement explicit server panels

- Local timestamp / acting role / milestone: 2026-11-08T18:57:14.589741-05:00 / designer / explicit-server-panels.
- Requested scope: User-selected explicit Console/tools opens, selected controls pressed inward without glow, redundant restart removal/Shift-click force requirement, and Terminal right alignment in Figma.
- Starting repository state: main72a6a33 with local documentation of the cancelled/reverted scroll trial; source unchanged.
- Work performed / changed files: Root updated shared default output visibility, removed two redundant Recovery buttons, created four prototype counterparts, wired individual controls, added shared Selected button with inner shadows only, documented Shift-click limitation, and widened both Terminal masters to FILL1072. Root serializes this report into project docs/journal.
- Decisions: Keep work on new Figma design pages; retain original scroll and source/reference pages. No product API/runtime changes. Figma cannot simulate a modifier-click through its click trigger.
- Checks and evidence: Shared Selected172:1910 has two INNER_SHADOW effects and no DROP_SHADOW. Terminal77:1453/77:1469 both read back width1072 with FILL. Closed state output false; explicit Console-open counterparts true after applying inherited visibility overrides. Actual prototype verification pending.
- Resulting state / acceptance: Bounded plan implemented; independent prototype testing and evaluation pending.
- Remaining work and handoff: Test real button clicks and obtain independent final review, then update current records.
