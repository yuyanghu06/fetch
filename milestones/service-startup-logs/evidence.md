# Service startup logs — final evidence

November 4, 2026. Independent Evaluator ACCEPT.

## Result

The shared console displays actual service startup output/instructions and returned errors rather than synthesized authentication phase/host progress or the literal protocol-error summary. Local stderr, recorded setup stdout/stderr, nonprotocol stdout (still rejected), MCP notifications/message, error code/message/data, initialize instructions, failed HTTP status/body and OAuth failure stages all use one private attempt capture. Actual process exits/signals remain. Generic local API/read failures are fallback only until service output arrives; stale fallback entries are then removed. Existing host consent, Copy and shared console rendering remain.

One Runner registry holds latest per-instance capture independently of configured connections, with unique IDs and task-owned handles. Old attempts cannot freeze or append to a newer capture. Short protected-access probes borrow the OAuth capture without ending it. Capture retains the beginning up to1MiB/4096lines and64KiB perline with explicit notices. Assigned/request credentials, secret-bearing JSON keys/numeric values, nested echoes, embedded SDKJSON, bearer/cookie/auth/state/verifier values and sensitive URLs are sanitized before retention. Response-derived redactions are temporary/zeroizing rather than accumulating outside the log budget. No successful protocol/tool/token payload dumps, global SDK debug logging, disk logs or new API routes. Only native Settings receives logs; public/persisted errors and raw result forwarding keep their existing contract.

The project-wide thin-wrapper default is now durable in AGENTS.md, decisions.md, PROJECT_CONTEXT.md and NOTES.md. This logging change does not add a new authentication flow or infer/execute setup commands.

## Independent verification

184 unique tests pass:123library +61integration, including15new checks (10focusedlibrary +5integration). Final31UI checks pass after the last fallback-only correction; they repeat a subset of the184, not31additional unique tests. Locked native build passes (9.25secondsCargo/9.33secondswall). Whitespace passes.

```sh
cargo test --locked --lib
cargo test --locked --test service_startup_logs --test standard_authentication --test program_paths --test isolated_native_mcp --test raw_json --test backend --test import_compatibility --test direct_install
cargo test --locked --lib ui::auth_tests
cargo build --locked --bin fetch
git diff --check
```

Actual confined fixtures verify80startup lines including the beginning; declared Railway-shaped initialize instructions followed by tools/list error-32001 with original details; setup stdout/stderr/exit9; invalid stdout still failing; successful protocol results excluded and capture frozen; failed HTTP reply details. Manager/SDK checks cover unconfigured/stale attempt handles, native-only projection/no saved log, bounded prefix/truncation, nested/key/numeric credentials, embedded JSON in SDK error text, exact guard/registration errors and production302rejection visibility. Existing process/helper/remote/API/raw-forwarding/import/direct-install regressions pass.

Initial fixture modes named reject/invalid were intentionally redacted wherever those argument values appeared, masking words in assertions. Tests now use unique labels; product behavior was unchanged for that issue. Review found/corrected material sanitizer/SDKstatus/detail issues. A final UI test exposed stale fallback read errors after a successful poll; source now removes all prior Fetch fallback lines when actual service output appears. No unresolved material findings.

## Isolation and limits

Synthetic local children run only under existing verified deny-default macOS sandbox with outside-file/sibling/network canaries. Remote responses are disposable loopback/injected fixtures; grants use memory backend. No real provider account, personal credentials/settings, native Fetch app, signing/release or package installation occurred in this verification.

Separate actual Railway research is recorded in ../railway-codex-research and docs/architecture/railway-codex-startup-research.md: real shim/CLI initializes, then returns explicit-32001notloggedin. That proves a no-credential case, not this user’s account state or successful authentication. Hosted services’ internal logs cannot be displayed unless returned. Native window appearance/clipboard/relaunch/real provider sign-in remain unobserved.

Next user action: explicitly Quit Fetch, run ./scripts/run-development.sh and retry the desired server. No reinstallation is required for this logging change. Existing process/settings/state were preserved; no commit/push was performed.
