# Permanent expanded console — final evidence

November 4, 2026. Independent Evaluator ACCEPT.

Every expanded server renders the shared console, including empty, ready, stopped, failed and not-required states. Passive display and status polling do not start authentication or mark a server busy. Authenticate and manual save retain history. Collapse and window reopen restore it from the backend during the current Fetch run.

New capture handles inherit only sanitized output and prior observed exit facts. IDs, current exit metadata and credential-redaction state remain fresh; old handles are frozen. Existing 1 MiB / 4096-line limits bound cumulative history per server, with explicit notices. No log files or persistence across Quit were added. Delete and Quit retain their cleanup behavior.

The frontend renders the authoritative cumulative snapshot, so prior prefixes and exit facts do not duplicate. Log updates preserve manual-storage verdicts. A review correction records the output already present when a new request starts: unchanged old reads and carried old exit lines cannot hide a fresh local request failure. The fallback disappears only after genuinely new service output arrives.

Authentication, issuer checks, credential stores, account folders, public API fields and provider behavior are unchanged. No Codex code was imported.

## Independent checks

150 unique relevant tests pass, including eight new tests: 130 library, six service startup, ten standard authentication, two private-program and two confined native MCP checks.

```sh
cargo test --locked --lib --test service_startup_logs --test standard_authentication --test program_paths --test isolated_native_mcp
cargo test --locked --test standard_authentication
cargo build --locked --bin fetch
git diff --check
```

The first combined run passed all targets except one old standard-auth assertion requiring previous FATAL output to be erased. That expectation is superseded by the new retention requirement. The focused update requires the sanitized fatal/old exit fact once and preserves fresh-generation/current-exit/privacy checks. Its final target passes 10/10. Unchanged passing targets were not repeated unnecessarily. Native build and whitespace pass, with existing deprecation/dead-code warnings.

New coverage includes passive expanded consoles, Authenticate/collapse/window restore, manual-save verdicts, new request failures beside old logs, bounded/redacted history, frozen stale handles, exit facts once, and an actual confined initial app-startup-to-auth transition without reading its private logs beforehand. Rendering is unconditionally present inside expanded rows; no native visual assertion is claimed.

## Separate observation and limits

An initial fixture used an explicit failed Restart followed by Authenticate and received SERVER_BUSY. That admission behavior is unchanged and remains a separate unresolved observation. The requested initial app-startup case instead uses Manager::startup, waits for its actual failed result without reading the private log, and passes. This acceptance does not claim every restart/authentication path works.

Actual children ran only in the existing verified deny-default macOS sandbox with disposable state and memory grants. No real provider/account, native app, signing, release, personal settings, credential migration, commit or push action occurred. Native appearance and actual account acceptance remain unobserved.

Next: explicitly Quit Fetch, run ./scripts/run-development.sh, expand a server and retry its desired action.
