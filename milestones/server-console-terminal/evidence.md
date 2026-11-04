# Console/Terminal evidence — November 4, 2026

## Final result

Independent Evaluator ACCEPT.182 relevant distinct checks pass, including19new (terminal7, persistent log4, UI8). Locked native build passes. The user-provided screenshot established a blank native pane; the narrow embedding fix now has actual offscreen rendered-pixel evidence. The running native app has not been relaunched by the agent.

## Commands and outcomes

- `cargo test --locked --lib`:164passed,0failed. Final production source includes created/reused query handling, output drain, close/reap and local renderer geometry corrections.
- `cargo test --locked --test program_paths --test service_startup_logs --test standard_authentication`: initial run program_paths2passed, service_startup_logs5passed/1failed (SERVER_INFO_UNAVAILABLE at ordinary agent server projection); later binary did not run in that initial command.
- `cargo test --locked --test service_startup_logs --test standard_authentication`: final6+10passed. Exact Railway-shaped case then passed five isolated repeats. Initial failure retained; cause unconfirmed. Its fixture uses Runner::new with no terminal or disk-log writer, so no feature cause is established.
- Focused strengthened foreground-close test passes: actual distinct owned shell and foreground sleep PIDs gone, readers/waiters joined, PTY descriptors released. This is one of the164, not an additional unique test.
- Focused offscreen embedded rendering test passes: real Iced Headless GPU RGBA output contains prompt-only glyph pixels and separate cursor pixels at y430, same cached output moves to y90 without new bytes, old position is empty, viewport clips hidden glyphs. This is one of the164.
- `cargo build --locked`:PASS16.25s; existing warnings remain. No app launch/signing performed.
- `git diff --check`:PASS.

## Covered behavior

Real disposable shells run under deny-default macOS resource confinement with private fixture HOME/cwd/PATH/assigned environment; outside-file/network canaries are denied. Actual shell IO, resize, final trailing output/exit, live reuse, per-server/session boundaries and blocked-input close are checked. Account-shell selection is independent of an assigned SHELL value. GET/invalid/unauthorized routes do not start a shell; native capability and target permission stay required.

UI fixtures cover retained screens through Console/Terminal, collapse, page/window changes; stale response suppression; no uncertain input replay; Close independent of blocked writes; old-output gaps; fresh201 startup-query replies and reused200 query suppression. Terminal processes stay backend-owned; the existing iced_term/Alacritty implementation handles parsing/rendering/input.

Disk fixtures prove sanitized newest bounded history, shutdown flush/new-run reload before configure, exit fact carryover once, frozen old callbacks, private file mode, passive no-write reads and safe preservation of symlinks with honest failure notices. No terminal input/transcript enters console persistence.

## Original checked output

Root inspected these task-specific files (temporary logs, not permanent app state):

- /private/tmp/fetch-terminal-final-lib.log
- /private/tmp/fetch-terminal-integration.log (initial failure retained)
- /private/tmp/fetch-terminal-integration-retest.log
- /private/tmp/fetch-terminal-service-repeat.log
- /private/tmp/fetch-terminal-foreground-test.log
- /private/tmp/fetch-terminal-render-test.log
- /private/tmp/fetch-terminal-final-native-build.log

No fixture processes/folders remain. Test logs contain fixture data only. No personal credentials, provider accounts, personal MCP/client settings, or existing services were used/changed.

## Limits and next action

Rebuilt native appearance, real shell/account commands and real provider login remain unobserved. Explicitly Quit Fetch, then run `./scripts/run-development.sh` to load the correction and check its real window. Terminal cleanup owns the shell/PTY foreground group; detached background daemon enumeration is outside this contract. The transient ordinary server-read failure remains an observation with unconfirmed cause, rather than a claimed fix. No release, packaging or distribution verification performed.
