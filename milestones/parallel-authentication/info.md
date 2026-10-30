# Parallel authentication and checked credential batches

## Goal and scope

October 30, 2026: the user approved the implementation summary and asked for the
remaining MVP blockers afterward. Preserve POST authenticate's saved-credential
service check and D31's checked credential-save result. Different services should
authenticate independently; one service may require several supported values.

Backend work replaces the one-entry save shortcut with checks of the current
supported authentication references. UI work tracks requests per server, with
same-server exclusion and protection against old results. No new provider adapter,
route, storage-only deletion success or generic protected-MCP proof is selected.

## Completion criteria

- POST authenticate returns authenticated only after checking the saved current
  credential through the existing service-authenticated endpoint.
- Supported multi-entry authentication saves check all covered required values;
  a prior OAuth token cannot hide a newly submitted invalid manual key.
- Different servers' requests and UI outcomes are independent; same-server
  duplicates and old responses cannot overwrite current work.
- Independent fixtures cover the real backend/API path and UI state handling,
  the native binary builds, and a separate review accepts the bounded change.
- Record remaining provider, credential-removal and native/client testing gaps
  without claiming full MVP readiness.

## Status and boundaries

Complete for the bounded code/fixture scope after independent Evaluator ACCEPT.
Commit/push delivery is next. Backend and UI writers owned separate modules; the
Tester owned their new test files. The
Orchestrator owns documentation and serializes pass history. Tests use disposable
files, memory-held fixture keys and in-process mock services only. No live MCP,
provider, native window, Keychain, trust or personal-client changes are part of
this pass. Actual protected native installation/client/restart proof remains
pending under the parent local-working-MVP milestone. No release or packaging.

Source passes are implemented: backend compilation passed in 5.67s; UI compilation
passed in 4.75s. Independent fixtures now pass 25/25 and the combined suite 93/93; the native binary
build passes in 0.54s. Final evaluation ACCEPTs the scoped change; delivery is next.
A cfg(test)-only host factory now permits actual Desktop message testing without
opening a native window or using production Keychain state. See the rolling
snapshot for the serialized writer and Orchestrator pass entries.
