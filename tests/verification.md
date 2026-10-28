# Developer backend verification — October28, 2026

The independent Tester ran `./scripts/verify-dev.sh` on macOS after the reported fixes. It exited successfully.

| Check | Actual result |
| --- | --- |
| `cargo fmt --all --check` | Passed. |
| `cargo clippy --locked --all-targets -- -D warnings` | Passed. |
| `cargo test --locked --all-targets` | Passed:37 tests total—13 library tests and24 independently written integration tests. |
| `bash -n scripts/verify-dev.sh` | Passed. |

## Observable behavior checked independently

- Public resource metadata works through real HTTPS with normal certificate checking and no system-trust change. OAuth metadata reports unavailable while native approval/token issuance is incomplete.
- The test HTTP client rejects the untrusted certificate and the localhost certificate used with an IP address.
- The listener binds IPv4 loopback. A foreign browser origin or wrong/duplicate Host is rejected. HTTP/1.1 requires Host; HTTP/2 accepts its matching authority without a Host header. Forwarded headers cannot replace the configured authority.
- Missing authorization and missing management permission deny source/server information before disclosure. A pretend developer token grants no access.
- GET server ordering, repeated-ID deduplication, exact decoded IDs, missing IDs, invalid queries, nonempty bodies and malformed percent/UTF-8 input follow the expected contract.
- Corrupt saved data gives a safe read failure, rather than claiming there are no installed servers. Saved flags do not manufacture current runtime readiness or verified provider authentication.
- A filtered read of healthy selected servers succeeds even when an unrelated saved record is corrupt. An unfiltered read still reports that corruption safely.
- Restart lookups distinguish an unknown server, a known server with no matching attempt, invalid repeated selectors and an unreadable status record. Unknown-server replies preserve the missing ID; no different attempt is substituted.
- Discovery preserves safe supported siblings, disables selection for blocked/source-disabled rows, reports explicit-file errors separately from failed default sources, and does not return source environment values or shell commands.
- The approved read handlers also pass actual HTTPS requests using a fixture grant injected only by test code.
- A second developer instance cannot use the same state directory. A port conflict does not replace an existing public certificate. Restarting developer state creates a fresh certificate; only its public certificate is saved.
- Symlink or publicly readable developer-state directories are rejected. Raw JSON fixtures preserve unknown fields, large numbers and media data.

## Failures found and retested

The first completed run found that saved-record enumeration failed on macOS: a repository error also made the server-list API return500. The Coder replaced the `/dev/fd` enumeration with directory-descriptor enumeration. Both the library case and independently written HTTP-handler case now pass.

The full verification script then found five Clippy warnings in Coder-owned source. The Coder fixed them; the complete script passed afterward. The Tester did not edit application source.

The Evaluator subsequently requested selected-record isolation and precise restart-error handling. After Root's fixes, the Tester added two regression tests covering healthy reads beside unrelated corruption, unknown-server/missing-attempt/read-failure codes and repeated selectors. The complete verification script passed again with36 tests. The Coder's additional unreadable-default-source fixture also passed in that run.

The final review found that valid HTTP/2 authority-only requests were rejected. After Root corrected the address check, a new real-TLS regression negotiated HTTP/2, received200 and verified public resource metadata. The complete script passed again with37 tests.

## Limits

All files and keys were disposable fixtures. HTTPS trust was limited to the test HTTP client. No MCP server or provider was contacted; no personal client configuration, credentials, Keychain or system certificate trust was read or changed. No native UI, real app authorization, package preparation, production Keychain behavior or MCP client/provider compatibility was tested. Those remain separate implementation and later isolated-testing work.

## Development-port follow-up — 2026-10-28T13:06:54.850498-04:00

Updated only the six static backend test fixture references to the final developer port53671. HTTPS tests retain available port0; no fixed-port listener or MCP connection was started. `cargo fmt --all --check` and `cargo test --locked --all-targets` passed again:37 tests (13 library,16 backend,7 HTTPS,1 raw JSON). `cargo run --locked -- dev --help` exited successfully and displayed `--port` default53671. No new test or production setting was added by the Tester.
