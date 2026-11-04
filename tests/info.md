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

## October 30 authentication fixtures

New unit fixtures live beside their owned modules: src/manager/auth_tests.rs
contains 11 cases and src/ui/auth_tests.rs contains 14, including five real Desktop
message cases without windows or executed Tasks. All25 pass. A fresh combined
run passes93/93 and the native binary builds. See
[the scoped evidence](../milestones/parallel-authentication/evidence.md) for commands
and limits. No real provider/native/MCP/test-Codex acceptance follows.

D82 adds 17 independent manager cases across generic_auth_tests.rs and
verification_tests.rs, sharing the existing disposable auth harness. Final
35 targeted manager and110 combined cases pass, along with native compilation.
See milestones/generic-authentication/evidence.md for commands and review/retest
history. No live provider/native/MCP/client acceptance follows.


## November 3 automatic discovery

Seven new backend cases cover default discovery without a path, home/.codex folders, exact-file overrides, partial failures, empty/missing/symlinked folders, injected CODEX_HOME defaults and permission checks. Fixtures confirm known paths only, secret suppression and unchanged source bytes. Three new Desktop message cases in src/ui/auth_tests.rs cover first-visit discovery, explicit override/default reset and incompatible-operation guards. These 10 new cases and all 47 relevant backend/import/UI cases pass; native build passes. Desktop Tasks are dropped without execution; no actual native interaction or personal configuration is used.

## November 3 shared authentication router

The router's independent fixtures are in src/manager/router_tests.rs. They cover
config-driven selection, common web request forms, preserved compatibility,
Google's pending/no-execution result, isolated accounts of the same package,
visible background errors and issuer/attempt ownership. They use disposable
encrypted stores and loopback mocks; there are no real provider/helper logins.
The existing persistent-alias case in local_mvp_acceptance.rs now configures
stopped disposable routes after Runner restart before resolving aliases, matching
the existing deletion protection. Configuring does not execute the fixture path.
Current command/results are recorded in the shared authentication milestone.

## November 3 direct installation POST inputs

`direct_install.rs` contains8 disposable integration cases for command/url/source
selections, identity/reuse, saved credentials, HTTP authority/progress/errors and
partial failure. `src/installation/direct_tests.rs` contains6 pure capture and
supervised-hook cases; hooks simulate outcomes and no actual executable/provider
runs. New14 plus128 existing checks pass (142 unique); native binary builds.
See milestones/direct-install-inputs/evidence.md for exact commands and limits.

Merged-main follow-up: two additional installed-row checks now cover folder-discovered saved associations and native selection guards. Final integrated checks pass 64 distinct cases:24 backend,7 import,8 direct-install,18 UI and7 discovery units. This includes12 new cases for the feature; native build passes. Existing incoming deletion latency logs are preserved rather than rewritten for whitespace.
