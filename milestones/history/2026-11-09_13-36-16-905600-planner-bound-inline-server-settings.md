# Bound inline server settings

- Local timestamp / acting role / milestone: 2026-11-09T13:36:16.905600-05:00 / planner / inline-server-settings.
- Requested scope: Remove the standalone Settings page; put Settings inside each server card, remove navbar gear, then refine numeric labels/controls and light Copy.
- Starting repository state: Existing local design documentation and journal changes; no application source changes.
- Work performed / changed files: Independent Planner bounded state-preserving inline Settings families and correct Filesystem/Search identity. User clarified inside each card. Earlier fixed-Settings-navbar idea cancelled before implementation.
- Decisions: Figma new-design pages only; preserve original references, source, personal settings and backend behavior. Root serializes shared records.
- Checks and evidence: Source inspection confirms folder/call limits/manual keys are per server; no global-settings placement decision needed. Existing scrolled/filled credential editors compare identical.
- Resulting state / acceptance: Plan accepted; preserve existing field values/busy states and original references.
- Remaining work and handoff: Implement per-card states and apply user-requested field/copy refinements.
