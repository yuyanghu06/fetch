# Backend verification

These tests use temporary data, fixture configuration files and test-only authority objects. HTTPS tests open an IPv4 loopback listener on an available port and trust its public certificate only inside that test's HTTP client.

Most checks stay in-process. `isolated_native_mcp.rs` starts synthetic MCP peers
inside verified macOS resource sandboxes, and the explicit ignored
`real_native_mcp.rs` checks a separately staged original Everything package.
Neither uses provider accounts, personal configuration, system trust changes or
the real Keychain. Passing fixtures does not establish universal compatibility.

## Native server storage verification

Final November3 frozen-source run:151 checks passed, none failed, and the explicit
real-package case remained ignored in the ordinary suite. That real case also
passed separately, giving152 distinct exercised cases. The selected command used
`cargo test --locked --lib` plus backend, development_https, environment_paths,
import_compatibility, local_mvp_acceptance, raw_json, direct_install,
isolated_native_mcp and real_native_mcp targets. Native build and independent
evaluation are recorded separately by the Orchestrator.

`native_state/` contains ordinary HOME/XDG file fixtures with no credential
interception. Each peer must pass owned-file positive checks and outside-file,
sibling-state and TCP denial canaries before MCP execution. Tests cover raw large
numbers/media/argument bytes, explicit restart/file reuse, separate accounts,
other-server responsiveness and state-preserving registration removal. Owned
PIDs exit and temporary roots are removed after tests.

`real_native_mcp.rs` is ignored by default. It requires an explicitly marked task
root, public Node24.22.1 and original Everything2026.8.31 staged separately. It
does not install packages. Its runtime sandbox denies all network and external
file access. The November3 explicit run passed:13tools, echo before/after restart,
preserved package/native/legacy files on removal and confirmed owned child exit.
All task-created packages, runtime, caches and state were then removed.

`src/remote_oauth/tests.rs` uses fake Keychain storage and scripted SDK HTTP
responses. It covers instance/root/service/client binding, rotation, missing
replacement refresh tokens, storage failures, callback state/issuer rejection
and unmodified manual values. Its real local HTTPS test exercises Fetch's actual
restricted self-request sender and authenticate route, with a fake remote access
check; it does not establish real provider or Keychain compatibility. Repeated
status reads must not contact providers or wait for a blocked renewal.

`src/api/background_tests.rs` checks that background authority is process-owned
and cannot open login, run native setup, edit configuration/manual keys, invoke
tools or open Finder. Its fake-Mac-confirmed manual PATCH checks report stored
with verified:false, reject unassigned values and fail safely on missing Keychain
readback. This is test authority only, not a real Mac approval result.
`local_mvp_acceptance.rs` holds a rejected request using old headers until after
renewal, then checks that its late reply cannot invalidate replacement access or
replay a tool. `direct_install.rs` checks execution-only public edits and
opaque legacy bytes remaining unchanged during installation reuse.

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

## Opt-in deletion latency demo

delete_latency_demo.rs is ignored by normal test runs. It requires an explicit
fresh /private/tmp/fetch-delete-demo-* root and marker plus independently verified
OS confinement. It installs real Everything2026.8.31, verifies echo, times deletion
and checks package/config removal and preservation. Its fixture keys bypass the
OS credential store, so results do not establish native window or Keychain
acceptance. Never enable it as part of ordinary automated fixture runs.


## November 3 direct installation POST inputs

`direct_install.rs` contains8 disposable integration cases for command/url/source
selections, identity/reuse, saved credentials, HTTP authority/progress/errors and
partial failure. `src/installation/direct_tests.rs` contains6 pure capture and
supervised-hook cases; hooks simulate outcomes and no actual executable/provider
runs. New14 plus128 existing checks pass (142 unique); native binary builds.
See milestones/direct-install-inputs/evidence.md for exact commands and limits.

Merged-main follow-up: two additional installed-row checks now cover folder-discovered saved associations and native selection guards. Final integrated checks pass 64 distinct cases:24 backend,7 import,8 direct-install,18 UI and7 discovery units. This includes12 new cases for the feature; native build passes. Existing incoming deletion latency logs are preserved rather than rewritten for whitespace.

The Google-specific ignored case prepares @a-bonus/google-docs-mcp1.11.3 through
Installer::submit and deletes it through Manager, without Connect, Google server
execution, login or account calls. Run exactly one named ignored case per process
with its own fresh root; do not enable all ignored cases together. The Everything
case additionally connects and verifies echo. Google package cleanup evidence
does not establish working Google authentication.

November3 actual confined diagnostic results: Everything install/echo/delete PASS
(20.400s delete); Google package preparation/delete PASS (75.225s delete). Both
retain source/runtime/unrelated canaries during deletion. Independent evidence
review ACCEPTs cleanup latency diagnosis; all temporary environments were removed.
See [the evidence](../milestones/delete-server/latency-evidence.md). These opt-in
measurements do not replace native UI, Keychain, provider or separate-client checks.

November3 approved flush optimization: independent storage checks6/6 PASS
(two new cases in src/storage/deletion_tests.rs); confined Google deletion now
0.889s and Everything0.262s with matching baseline entry counts. Actual connection/
echo is verified for Everything; Google remains preparation-only. See
[optimized evidence](../milestones/delete-server/optimized-evidence.md).


## Historical shared helper credential storage

The removed core bridge_tests.rs and manager/helper_auth_tests.rs added21 checks around the
real encrypted store/shared channel/actual Google bootstrap with synthetic
modules. The former tests/helper_bridge documented the OS-confined Node peers and support.
176 unique selected checks pass; final managed-child test asserts raw results,
startup authentication and PID/socket cleanup. No real Google/native/account
execution. Initial sandbox/bootstrap and same-owner fixture failures were fixed
in tests only; exact commands and cleanup are in the shared authentication
helper-storage evidence.

## November3 — final native-storage verification

Frozen-source relevant suite151PASS/0fail/1ignored; explicit original Everything
confinement check separately1PASS. Final build passes; independent Evaluator ACCEPT.
Exact commands/counts/corrections/cleanup/limits are in
[milestone evidence](../milestones/native-server-storage/evidence.md).
