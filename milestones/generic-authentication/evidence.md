# Generic authentication evidence

October 30, 2026. Independent Tester results inspected by the Orchestrator.
All commands use process-scoped `DEVELOPER_DIR=/Library/Developer/CommandLineTools`;
no system toolchain/license setting changed.

| Check | Actual result |
| --- | --- |
| `cargo check --locked --lib --message-format short` | Initial source passes in 8.23s; later lifecycle fixes are compiled by final tests/build. |
| `cargo test --locked --lib manager::` | Initial32/32; final 35/35 after three regression cases. |
| `cargo test --locked --lib --test backend --test development_https --test environment_paths --test import_compatibility --test local_mvp_acceptance --test raw_json` | Preliminary107/107; final110/110 (lib64,backend16,HTTPS7,paths4,imports7,MVP11,raw1). |
| `cargo build --locked --bin fetch --message-format short` | Final exit 0;0.43s. |

The 17 new cases prove arbitrary names from disposable configs reach server-local
encrypted storage and generic environment/header route delivery, without source
secret copying or false authentication. A trusted custom description combines
two arbitrary saved inputs in one actual mock request. Other cases cover 2xx
semantic rejection, exact Linear/Railway predicates, missing/ambiguous/unused
inputs, request destination conflicts, malformed later requests failing before
any dispatch, caller-supplied verifier refusal, production login-mode selection,
manual remote proof without an OAuth record, and unrelated-header exclusion from
authentication evidence. Prior authentication/parallel UI checks remain passing.

Independent review found material login-selection defects despite the initial
passing suite: profiles could disable generic OAuth and unrelated header values
could be mistaken for authentication. The writer fixed them, the Tester added
regressions and final checks pass. No automated test failed during this pass.

Four existing Security bridge deprecations and the unread MCP error field warning
remain. Complete final test output was inspected from
`/tmp/fetch-generic-auth-final-tests.log`; this summary is the durable evidence.
Mocks/disposable memory keys only: no real provider, Keychain/native window, MCP
installation or separate test-Codex call/restart ran. This is not full MVP readiness.

Final independent Evaluator decision: **ACCEPT**, after code, final test/build
evidence and accurate remaining-limit documentation were reviewed.
