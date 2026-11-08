# Plan current Fetch Figma sync

- Local timestamp / acting role / milestone: 2026-11-07T13:54:40.667832-05:00 / Planner and Orchestrator / current-figma-sync.
- Requested scope: Copy the current native app UI into the official Figma file.
- Starting repository state: Clean working tree; existing Figma screens reflect October 28 source.
- Work performed / changed files: Read-only specialist inspected all native views, components, authentication and terminal rendering. Orchestrator inspected current Figma pages, components, fonts and libraries. Documentation initialized for this pass.
- Decisions: Preserve current app layout and actual labels; reuse existing local Iced components rather than incompatible macOS library buttons. Use disposable, window-free UI fixtures for native rendering references. Keep old states visibly historical where removed by current source. No product behavior or API changes.
- Checks and evidence: Existing native client size1080×760; padding28; navigation gap12; output height200; current source has no tagline/cleanup form/JSON configuration editor. Figma SF Pro Regular is available, unlike prior pass. App is not running. No runtime/account/MCP connection work performed.
- Resulting state / acceptance: Plan accepted for design-only work. Shared workflow/role files referenced by user are absent locally; specialist roles assigned explicitly using available collaboration tools.
- Remaining work and handoff: Render safe reference screens, update components and editable pages, compare screenshots, independently review and record actual limits.
