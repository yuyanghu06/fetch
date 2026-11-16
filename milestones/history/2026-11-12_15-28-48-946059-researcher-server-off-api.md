# Research server off operation

- Local timestamp / acting role / milestone: 2026-11-12T15:28:48.946059-05:00 / Researcher, parent-serialized journal / server-on-off.
- Requested scope: inspect reusable stop paths and the user's allowed extension of the existing start/restart POST.
- Starting repository state: no stop route or durable off flag; broad existing dirty tree and running Fetch preserved.
- Work performed / changed files: specialist read-only source inspection; parent writes milestone/notes records only. Verified route/Manager restart validation, Runner disconnect, scheduler admission, startup/background/auth paths and catalog notifications.
- Decisions: user approves presented stop contract and selects durable Off via asynchronous question. Research confirms no stop API. Existing disconnect/close code can be reused with an idle admission guard, and Stop must run before remote renewal. Durable Off needs guards across startup, installation follow-up, authentication, renewal/preflight and delayed restarts. User now explicitly selects durable Off and the presented action:stop extension with non-interrupting busy rejection. Planner will bound implementation; no stop source written.
- Checks and evidence: source inspection only, no research builds/tests or real MCP/provider calls. Key refs: src/api/runtime.rs:75, src/manager/mod.rs:222,297,329,451; src/runner/mod.rs:365,452; src/runner/scheduler.rs:139; src/storage/mod.rs:133,209.
- Resulting state / acceptance: research accepted for planning; disconnect alone would interrupt work and persistent off needs all start-capable paths covered. No application stop code yet.
- Remaining work and handoff: planner then coder/tester/evaluator. Shared workflow/role files absent; do not install or recreate configuration. Parent preserves all source and serializes journals.
