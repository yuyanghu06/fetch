# Refine server state and footer layout

- Local timestamp / acting role / milestone: 2026-11-07T19:44:26.505320-05:00 / designer, recorded by root / neumorphic-figma
- Requested scope: Live user feedback on authentication availability, dot alignment, optional gear, gray output controls, duplicate ID and inline footer; explain separate tools section.
- Starting repository state: Previous compact header/action slice accepted; existing earlier source change and unrelated files preserved.
- Work performed / changed files: Optically aligned dot wrappers in8header variants; lightened Authentication Disabled variant to#B8BDC3. Added a per-server gear then removed it at latest explicit user request. Three actions remain on13headers. Removed redundant filesystem IDs from7expanded masters. Console/Terminal14native buttons now#E3E6E9. Created shared data-driven footer139:1940 with three text properties; replaced7footers and restored exact values. Library121local mains/14sets. Figma-only. Root updated docs/notes/journal/evidence.
- Decisions: Per-card gear cancelled, navbar gear retained. No new architecture/API or native execution. Tool-definition move is a recommendation only. Authentication gray follows actual existing auth fields rather than runtime inference.
- Checks and evidence: Root structural evidence13headers/three controls each,0per-card gears/IDs,7footers with right edge at1072,14gray output buttons. Final screenshots under milestones/neumorphic-figma/evidence/server-header*.png. Source rendering order inspected in src/ui/mod.rs. Official auth source: https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization . Independent final ACCEPT.
- Resulting state / acceptance: Selected Figma refinements complete.
- Remaining work and handoff: User review; possible tool definitions move into owning card; later selected Rust UI port. No application/service/account launch, tests or build needed for this design-only change.
