# Private program folders — verification

November 4, 2026. Scoped startup repair accepted.

Installer records relative executable folders privately. Shared native transport adds existing contained folders before configured PATH or the original default. Older installations use the same installer-owned known layout in memory without reinstalling or writing records. Startup and recorded setup share this environment. No new login code, public field, global environment inheritance or recursive scan.

## Independent tests and build

60 unique tests pass, including 10 new checks.

| Suite | Passing checks |
| --- | ---: |
| Installer program paths | 6 |
| Manager native storage/startup | 11 |
| Installation direct definitions | 6 |
| Storage | 6 |
| Confined helper lookup | 2 |
| Standard authentication | 10 |
| Confined native MCP | 2 |
| Direct installation | 10 |
| Import compatibility | 7 |

Successful commands:

```sh
cargo test --locked --lib program_paths -- --nocapture
cargo test --locked --test program_paths -- --nocapture
cargo test --locked --lib manager::native_storage_tests -- --nocapture
cargo test --locked --lib installation::direct_tests
cargo test --locked --lib storage::
cargo test --locked --test standard_authentication --test isolated_native_mcp --test direct_install --test import_compatibility
cargo build --locked --bin fetch
```

Locked native build passes (0.47 seconds reported by Cargo, 0.52 seconds wall). Final regression command took 37.29 seconds, including an intentional 31.25-second wait. Orchestrator git diff --check passes. Independent Evaluator: ACCEPT.

## Actual process evidence

A disposable legacy Railway-shaped record starts a confined synthetic MCP that invokes a helper by bare name. It succeeds, serves a real MCP tool call, and leaves legacy metadata unchanged. A second installation cannot find that helper. A recorded setup command uses newly saved private folder metadata (including a path with spaces) and runs the helper. Saved metadata survives repository round-trip and stays out of public repository/manager results.

Each child runs through the existing macOS deny-default sandbox after owned read/write and forbidden outside/sibling/network canaries. Fixtures assert temporary state removal after shutdown. No personal CLI, Railway account, provider login or native app was executed.

Initial integration run found the helper but the test called a tool before Restart had completed, receiving SERVER_BUSY. The test now waits for the restart operation to succeed; final 2/2 checks pass. No product-source fix was needed.

## Files and practical limit

Source: src/installation/program_paths.rs, installer metadata in src/installation/mod.rs, shared PATH construction in src/manager/native_state.rs, private field validation in src/storage/mod.rs. Tests: src/installation/program_paths_tests.rs, src/manager/native_storage_tests.rs and tests/program_paths.rs.

Existing Railway needs no reinstall. Next user action: explicitly Quit Fetch, run ./scripts/run-development.sh, and retry Railway Authenticate. The app was not relaunched or signed in this pass; real Railway sign-in remains untested. Existing settings, credentials, services and other dirty changes were preserved. No commit/push or release was performed.
