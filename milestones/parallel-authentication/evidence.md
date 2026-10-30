# Authentication fixture and build evidence

October 30,2026. Independent Tester reports, inspected by the Orchestrator and
reviewed by a separate Evaluator. This is bounded source/fixture evidence.

| Check | Actual result |
| --- | --- |
| `cargo test --locked --lib auth_tests -- --nocapture` | Initial17/17 pass; expanded final25/25 pass. |
| `cargo test --locked --lib --test backend --test development_https --test environment_paths --test import_compatibility --test local_mvp_acceptance --test raw_json` | Exit0;93/93 pass: lib47, backend16, HTTPS7, paths4, imports7, local MVP11, raw1. |
| `cargo build --locked --bin fetch --message-format short` | Exit0; Cargo reports0.54s. |

The25 new cases verify POST HTTP waits for the saved-key authenticated check,
rejected/replaced keys, all supported batch values, deduplicated shared references,
missing values, OAuth shadowing, unbound/removal failure, independent services,
same-service serialization, pending-browser repeat reuse and stale UI responses.
Five tests exercise actual Desktop message handling, including Settings saves,
without opening a native window or executing Tasks. Closing cleanup is called
directly and old-window replies are checked; the native Closed/picker bridge is
not exercised.

Credential stores and provider services are disposable; keys stay in test memory.
Test-only HTTP destinations are absent from production configuration. Save tests
call the private post-confirmation helper while holding its server lock; they do
not prove Mac confirmation. Existing fixtures retain the Settings-only boundary.
Four existing macOS Security API deprecation warnings and the unread MCP error
field remain; no new warning category appeared. No tests failed in this pass.

No real provider login, Keychain, Fetch window, MCP installation, separate test
Codex connection or restart/reuse was performed. These checks do not complete
the parent local-working-MVP milestone.

Final independent Evaluator decision: **ACCEPT** for this scoped source/fixture
change after reviewing code, actual test/build evidence and corrected blockers.
