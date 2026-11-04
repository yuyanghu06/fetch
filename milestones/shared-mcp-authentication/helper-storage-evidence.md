# Shared helper credential storage verification

November3,2026. Shared mechanism and Google1.11.3 adapter are implemented.
Independent Tester/Evaluator ACCEPT, no material findings. This is fixture/build
acceptance; real Google consent, installed-helper execution, native approval,
Keychain and actual personal-account access were not tested.

All Cargo commands used process-scoped
`DEVELOPER_DIR=/Library/Developer/CommandLineTools`, without changing Mac settings.

| Command/check | Actual result |
| --- | --- |
| Initial `cargo test --locked --lib credentials::bridge_tests` |8 pass/1 fail;30.01s. Confined Node bootstrap aborted before client execution. |
| Initial focused `credentials::bridge_tests::confined_node_client_uses_private_channel_without_polluting_raw_stdout_or_token_files` |0 pass/1 fail;1.31s; sandbox-exec SIGABRT6 also reproduced with true. |
| Corrected same focused client check |1/1 pass;1.43s after literal root-directory read and narrow executable mappings. Outside file read/write and loopback TCP canaries still denied; own files and exact bridge UDS work. |
| Initial `cargo test --locked --lib manager::helper_auth_tests` |9 pass/1 fail;2.22s. Reverify fixture assumed cached access although a new owner intentionally forces refresh. |
| Corrected focused `manager::helper_auth_tests::failed_reverification_clears_persisted_verified_marker` |1/1 pass;1.65s after using an existing owner. No production fix for fixture failures. |
| Final `cargo test --locked --lib --test backend --test development_https --test environment_paths --test import_compatibility --test local_mvp_acceptance --test raw_json --test direct_install` |176/176 pass: lib122,backend16,HTTPS7,paths4,imports7,MVP11,raw1,direct8; compile10.91s, library30.05s including the real30s request-timeout case. |
| Final `cargo test --locked --lib manager::helper_auth_tests::managed_stdio_attaches_bridge_preserves_raw_mcp_and_closes_child_and_socket` |1/1 pass after added same-test startup-auth assertion; compile6.45s/tests1.71s. Not another unique test. |
| `cargo build --locked --bin fetch --message-format short` |Exit0,0.48s; existing five native Security deprecations and unread MCP error-field warning remain. |

New21 cases (bridge9/helper12) cover encrypted readback ACK, conditional writes,
old proof invalidation, account isolation, unrelated secret-field preservation,
revoked/stale contexts, exact process-session failure guards, initial refresh-only
state, runtime verification-before-ACK and rotated-token preservation. Helper cases
exercise pending ownership, parallel account outcomes/other reads, saved-access
failure, fresh explicit retry, current app-input proof, execution-only settings,
shutdown and startup classification. The synthetic ManagedStdio peer exercises
helper_transport -> Runner configure/connect/catalog/call/disconnect, verifies raw
large-number/media bytes and asserts child PID/socket cleanup.

Tests use synthetic helper/OAuth modules around Fetch's actual bootstrap with
only the fixture dependency hash replaced through cfg(test) launch override.
No upstream helper code is copied. The real installed1.11.3 auth/index and actual
nested OAuth2 dependency were inspected/hashed separately. No real provider or
credentials were used; the actual callback and account login remain unverified.

Node runs in a macOS deny-default sandbox with fresh HOME/empty environment,
owned files, system runtime mappings and one task-owned Unix socket. Outside-file
read/write and TCP denial canaries run before fixture code. No Keychain/Mach
service grant, personal config, general network or browser operation is allowed.
Only the public Node24.22.1 binary is copied read-only into disposable roots.

Tester verified zero task fixture/probe/outside/bridge directories remain under
/private/tmp; copied runtimes, canaries and sockets are gone. All probe processes
exited, and managed/helper PID disappearance is asserted. No personal process
inventory or pre-existing state was changed.

Review corrections were production fixes to the selected contract: permanent
owner revocation, timeout failure reporting, persisted proof clearing, forced
current app-input proof, execution-only continuity, explicit retry, config error
ordering and session-fenced late failures. No optional features, new public auth
route/UI, separate OAuth engine or plaintext token-file fallback were added.


## Integration with newer main

First push found newer remote8f2a118 (automatic config discovery). Separate Git
worktree preserved all primary dirty changes. Helper source cherry-picked without
changes; four documentation conflicts retain both records. Independent comparison
verifies18 helper source/fixture blobs and8 incoming discovery blobs unchanged.
Delivered d19d891; live origin/main matched after push.

Same full Cargo command above, with process-scoped
`CARGO_TARGET_DIR=/Users/yuyang/Documents/fetch/target` in the integration checkout:
188/188 PASS (lib126,backend24,HTTPS7,direct8,paths4,imports7,MVP11,raw1).
Compile21.07s, library30.01s, total55.08s. Native build exit0,0.54s, existing
warnings only. This includes all21 helper cases and newer8 backend/4 UI fixtures.
Tester/Evaluator ACCEPT. Zero owned fixture/probe/outside/bridge/evidence roots
remain; owned child-exit and socket checks pass. Actual Google consent/account
and native approval remain untested. No personal state/settings were changed.
