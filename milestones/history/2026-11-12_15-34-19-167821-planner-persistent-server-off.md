# Plan persistent server off and native toggle

- Local timestamp / acting role / milestone: 2026-11-12T15:34:19.167821-05:00 / Planner, parent-serialized journal / server-on-off.
- Requested scope: standard team implementation of the user-approved action:stop POST extension and durable native toggle.
- Starting repository state: no stop route, read-only research accepted, black compact auth UI complete; broad unrelated work preserved.
- Work performed / changed files: Planner read-only source review and bounded plan; parent writes canonical server-on-off contract, indexes, current notes and milestone records. Planner accepts the selected existing-POST action:stop design: optional config.enabled defaults true for legacy records; idle-only, admission-fenced stop saves Off before confirmed close; explicit current restart enables; automatic paths never clear Off. Same backend powers MCP action and native toggle. Exclusive production/test ownership is assigned. No material product choice remains.
- Decisions: stop before renewal, no-force/busy rejection, retryable confirmed close, saved disabled flag before close, explicit-vs-internal starts, per-server visible errors; no new route or config PATCH field. User already approved meaningful contract/persistence; no further confirmation needed.
- Checks and evidence: inspected actual files and Researcher refs, plan describes focused confined route/storage/startup/background/MCP/widget checks. No stop build/test/runtime performed. Baseline captured privately for scoped review.
- Resulting state / acceptance: bounded plan accepted for Coder. Production and test files have exclusive ownership; parent serializes journals.
- Remaining work and handoff: Coder implement; Tester verify own tests; independent Evaluator; fix/retest then integrate. Preserve all prior source, black app auth controls, services, credentials and personal configuration.
