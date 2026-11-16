# Coordinate shared build dependency

- Local timestamp / acting role / milestone: 2026-11-12T16:34:18.609410-05:00 / Orchestrator / connector-subscriptions
- Requested scope: Deliver signed connector fix after actual maintainedSDK RED/GREEN and19checks.
- Starting repository state: Connector fix verified; concurrent project support entered storage/manager after the tests compiled.
- Work performed / changed files: The SDK GREEN/.03second and all19connectorchecks pass. Signed build attempt then caught concurrent project-support source mid-edit: storage references unexported crate::projects; root added only the mechanical pub mod projects declaration for the already-authored data registry. No routes or auth were added. Retry exposed manager/project_scope.rs calls to missing Caller::project_id. Root inspected that task's explicit user-authorized-core/route-review-pending notes and sent a build coordination message to the existing Workspace/project task, preserving their source rather than inventing project authority or reverting work. Connector source remains independently clear; final signed build/evaluation pending coherent shared source.
- Decisions: Module export is mechanical data-core wiring, not project API/auth approval. No Caller authority semantics guessed; no unrelated source reverted or running app restarted.
- Checks and evidence: /tmp/fetch-subscription-signed-build.log E0433 then E0599; coordinated existing task, no new coding task or live client launched.
- Resulting state / acceptance: Connector source/test acceptance clear; integrated signed build still pending other task's safe compile wiring.
- Remaining work and handoff: Wait for owning task to complete interfaces, then rebuild/strict-verify and record final acceptance. User must not be told new executable is ready before that passes.
