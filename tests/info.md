# Backend verification

These tests use temporary data, fixture configuration files and test-only authority objects. HTTPS tests open an IPv4 loopback listener on an available port and trust its public certificate only inside that test's HTTP client.

They do not connect MCP servers, contact providers, read personal client configuration, change system certificate trust or access Keychain. Passing these checks does not establish MCP or client compatibility.

Run `cargo test --tests`. The full developer check is `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, then `cargo test`.

The four tests in environment_paths.rs separately exercise development/release
path selection for multiple supplied homes, distinct ports, current-build
defaults and invalid relative homes without creating app state or changing HOME.
They were authored independently. All four passed during the October 29 D81
fixture run; no app state was created and HOME was not changed.

`local_mvp_acceptance.rs` adds independent in-process MCP/API acceptance cases:
native protocol initialization and framing, permission checks before dispatch,
revoked sessions, duplicate JSON rejection, Settings-only secret writes, raw and
partial catalogs, persistent aliases, exact resource selectors and stopped-server
behavior. All files are disposable fixtures; there is no real MCP connection,
managed executable, native confirmation or Keychain access. All 11 cases passed
in the October 29 coordinated D81 fixture run. Source/fixture acceptance does not
establish real provider, native window or separate-Codex compatibility.

The first D81 run exercised 67 distinct cases: 66 passed and one import test
failed because its broad `/mcp` substring assertion also matched safe hostnames
such as `mcp.miro.com`. After its author corrected that assertion, the independent
targeted rerun passed all seven import cases. That first result covered 67 cases
across separate target runs. After the issuer-response fix added one library
case, a fresh combined run passed all 68 selected cases with exit 0, including
all seven disposable HTTPS cases. Earlier failure/retest evidence is preserved.
See [actual fixture evidence](../milestones/local-working-mvp/evidence/fixture-checks-2026-10-29.md).
