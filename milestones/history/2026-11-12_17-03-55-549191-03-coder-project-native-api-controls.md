# Native project controls wrap the APIs

- Local timestamp / acting role / milestone: 2026-11-12 17-03-55-549191 America/New_York / UI Coder / project-connections.
- Requested scope: Expose registration, independent view/install/connection choices, native server scope and clear project approval.
- Starting repository state: Workspace existed; no native project control state.
- Work performed / changed files: Added ui/project_scope.rs/tests; updated NativeClient, Workspace, Desktop and connector_setup. Explicit selected-folder registration, Shared/All/project filter, installation destination, Settings assignment and separate connection snippets. Typed project context follows native claim/save/reconcile. Popup shows actual project path with opt-in history; minimum viewport scrolls content with fixed actions.
- Decisions: Browsing never changes a running client connection. Claude command uses project scope; Codex project TOML is copied for the user to apply. Reuse current theme/buttons/pickers/surfaces.
- Checks and evidence: Two new actual-widget cases pass; six popup checks and existing connect navigation pass. Six relevant PNGs inspected, including760x520 popup. Initial String/private-field/non-Send compiler issues fixed before final passes.
- Resulting state / acceptance: API-wrapped controls implemented; no native app launch or actual clipboard/client configuration application.
- Remaining work and handoff: Manual user/client acceptance remains explicit; final source evaluator accepts.

