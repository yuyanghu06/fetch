# Wrapper-authentication evidence — November 4, 2026

Implemented: remove six Fetch-imposed compatibility restrictions in remote OAuth preparation, saved metadata reuse and callback query handling. The MCP library owns discovery/fallback and callback validation. Keep actual-request network rules, browser consent, endpoint/saved issuer/client identity, account separation, state/deadline and existing no-silent-deletion contracts.

## Independent checks

| Command | Result |
| --- | --- |
| cargo test --locked --offline --lib remote_oauth | 26/26 pass |
| cargo test --locked --offline --lib api::background_tests | 6/6 pass |
| cargo test --locked --offline --lib manager::native_storage_tests | 11/11 pass |
| cargo build --locked --offline --bin fetch | Pass, exit0,21.68s |
| Relevant source/test/documentation git diff --check | Pass |

43 unique fixtures passed before subsequent concurrent terminal/logging edits. False and missing advertised callback-issuer support complete library sign-in without iss; SDK legacy fallback works without issuer metadata; unused registration endpoints no longer block saved reuse; harmless metadata updates preserve saved access. SDK state/issuer checks, explicit advertised-support missing-issuer rejection, instance/endpoint/client separation, request destination rules and redaction remain covered. Callback route ignores provider extensions and rejects duplicate state/code/error/iss. Tests use fake HTTP, fake Keychain and disposable in-process fixtures, not live providers or personal state.

The initial targeted run passed24/25: a new fixture wrongly assumed registration persisted a grant before callback. Corrected the test expectation; final26/26 pass. No production auth fix was needed after tests.

## Build retries and separate console-test limitation

cargo build --locked --offline --bin fetch first failed when concurrent runner startup diagnostics referred to an unsaved log_store module. After that module was saved, retry failed in separate terminal work: vendor/iced_term/src/backend.rs:156, E0596, cannot borrow notifier mutably through a shared reference. At that time runner also declared log_store_tests before its file was present, preventing another library retest. The auth Tester/Orchestrator did not edit those other-owned files or dependency manifests.

After the other owner corrected logging/vendor integration, the current-tree native build passed (exit0,21.68s, existing warnings). Final independent Evaluator ACCEPTs the scoped authentication change; no auth correction requested. Authentication source/tests are unchanged since43 passing checks. A repeat of library tests against the newest concurrent source is still prevented by the other owner's declared-but-unsaved log_store_tests module; this separate console wiring is outside auth acceptance. No real Vercel sign-in is claimed.

## Handoff

The requested authentication removal is complete. Separate console work should finish its test wiring; a later combined-tree rerun belongs with that integration. For real sign-in, explicitly Quit Fetch and rebuild/reopen through ./scripts/run-development.sh before retrying. No app launch, real account test, personal client setting edit, signing, release, commit or push occurred in this pass.
