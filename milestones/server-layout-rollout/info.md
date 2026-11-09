# Approved Servers layout rollout

Goal: apply the user-approved recessed layout to every new-design Servers state.

Scope:15 existing Servers specimens on app page75:2718; other pages/source references unchanged. The later inline Settings task expands the Servers inventory to22 and removes the separate new-design Settings screens.

Implementation: uniform1248x1160 windows and1328x1240 stages,1168px Installed servers section with24px padding, shared recessed surface197:4166/warm-gray token, and1120px raised server cards. Approved tools grid/read-only values remain shared. Initial15 stages packed in free two-row canvas space; later22 use11columns/two rows.

Acceptance: consistent layout, preserved state text/data/actions/visibility, no cutoff/overlap, and independent review.

Evidence: [before/after state preservation](state-preservation.json) records zero text/reaction/output differences at rollout time, before subsequent user-directed navigation/Settings changes. [Plan](../history/2026-11-09_13-36-16-904879-planner-bound-approved-server-layout-rollout.md); [implementation](../history/2026-11-09_13-36-16-905305-designer-rollout-approved-server-layout.md); [test](../history/2026-11-09_13-36-16-905407-tester-verify-server-layout-rollout.md). Independent structure/rendered-appearance PASS. Actual post-rollout clicks not repeated because the user was actively using Figma.

Status: implemented and checked; independent final review ACCEPT with [inline Settings](../inline-server-settings/info.md). [Review](../history/2026-11-09_13-38-31-278029-evaluator-accept-inline-settings-and-rollout.md). No Rust/runtime changes.
