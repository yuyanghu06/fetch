# Normal system home conditional plan

- Local timestamp / acting role / milestone: 2026-11-04T17:41:29.250820-05:00 / planner / normal-system-home
- Requested scope: Plan generic HOME/Keychain compatibility change without implementation before the pending shared-login choice.
- Starting repository state: Shared transport forces private HOME/XDG; actual Codex builder otherwise inherits allowed host variables. Concurrent completed normal-PATH/terminal source is preserved.
- Work performed / changed files: Planner read current source and fixture code; no edits/execution. Root inspected matching source and records report.
- Decisions: Conditional implementation removes forced HOME only, retains protected assigned-HOME skip, private XDG paths/packages/cwd and existing PATH. Rename private state-folder helper/local to avoid claiming it is child HOME. No provider branch, GH_CONFIG_DIR, new keychain/auth logic or dependency.
- Checks and evidence: Update manager/native_storage_tests.rs and tests/program_paths.rs HOME/XDG expectations; terminal re-exec must pass fake HOME. tests/real_native_mcp.rs has a pre-sandbox HOME PID write: replace with explicit fixture path and require disposable test HOME before future opt-in execution. Standard-auth/service-log peers use explicit argv state paths; isolated_native_mcp, terminal tests and Sandbox canaries already have explicit disposable HOME. Compile executables then run with subprocess-local task-owned HOME, without global env mutation or personal Keychain use.
- Resulting state / acceptance: Plan ready, not implemented. Actual user repair still needs retry; fixtures can establish passed startup context only. Pending user choice is required to replace earlier private-home/account boundary.
- Remaining work and handoff: Await normal-HOME/shared-login answer. Then assign exclusive coder/tester files, independent build/tests and evaluation; preserve unrelated work.
