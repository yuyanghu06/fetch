# Check authentication versus running state

- Local timestamp / acting role / milestone: 2026-11-07T19:44:26.505319-05:00 / researcher, recorded by root / neumorphic-figma
- Requested scope: Live user feedback on authentication availability, dot alignment, optional gear, gray output controls, duplicate ID and inline footer; explain separate tools section.
- Starting repository state: Previous compact header/action slice accepted; existing earlier source change and unrelated files preserved.
- Work performed / changed files: Read the official MCP2025-11-25 authorization specification, especially transport scope and401 handling; inspected src/ui/authentication.rs observe() and docs/architecture/native-server-storage.md. HTTP OAuth authorization is optional; stdio has separate credential handling. Existing Fetch state distinguishes authenticated/not_required/required/failed/unknown/authenticating; running alone is insufficient. UI graying uses known auth status. No new protocol/API behavior selected. Root updated docs/notes/journal/evidence.
- Decisions: Per-card gear cancelled, navbar gear retained. No new architecture/API or native execution. Tool-definition move is a recommendation only. Authentication gray follows actual existing auth fields rather than runtime inference.
- Checks and evidence: Root structural evidence13headers/three controls each,0per-card gears/IDs,7footers with right edge at1072,14gray output buttons. Final screenshots under milestones/neumorphic-figma/evidence/server-header*.png. Source rendering order inspected in src/ui/mod.rs. Official auth source: https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization . Independent final ACCEPT.
- Resulting state / acceptance: Selected Figma refinements complete.
- Remaining work and handoff: User review; possible tool definitions move into owning card; later selected Rust UI port. No application/service/account launch, tests or build needed for this design-only change.
