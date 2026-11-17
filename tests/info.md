## November 17 — copied subagent name

The native Workspace API fixture checks that a copied card's fallback fills a missing cross-app name, preserves explicit names, and still reports a missing description. The Widget fixture in src/ui also converts and creates the Claude file from a Codex card whose displayed label differs from its file name.

## November 17 — create-only subagent save

The Workspace HTTP fixture attempts to add a subagent with an occupied filename, confirms the original bytes, and saves the draft as a new file. It also checks that ordinary Edit still updates its file and no temporary file remains.

## November 13 — native scroll phases

native_scroll_phases.m includes the Fetch bridge and verifies phase ordering, fresh finger starts and continued ownership through momentum. It never creates a window, installs the event monitor or opens accounts/Keychain. Compile with ARC/blocks and AppKit, Foundation, Security and LocalAuthentication. UI routing/undo/parser fixtures remain in src/ui; evidence is under milestones/workspace-editor-gestures/evidence.

## November 12 — hidden file-picker reuse fixture

native_file_picker.m constructs the bridge's hidden prepared panel twice, asserts object identity/no visible dialog/no active selection, and reports construction versus cache time. Compile with ARC/blocks and AppKit, Foundation, Security and LocalAuthentication. It never presents a chooser, selects a file or invokes Keychain/authentication functions. See milestones/workspace-session-popup/evidence/native-picker.txt.

project_registry.rs checks canonical persistent registration and scoped preparation
without any connection hook. project_scope.rs exercises two projects through
native HTTP and MCP with synthetic in-process loopback peers: Shared+own servers,
post-assignment denial, busy guards, histories, progress and legacy session replay.
OAuth/profile/activation/UI fixtures remain in their source modules. All use
disposable state and fake keys; actual clients/accounts remain manual acceptance.

workspace.rs uses disposable projects/app roots to verify existing-file saves, cross-app conversion, project-specific plain/compressed chunks, native-only API access and private model-cache metadata exclusion. It never reads personal state or starts real coding apps. UI fixtures live in src/ui/workspace_tests.rs.

## November 12 — server on/off product checks

server_power.rs reuses the existing confinement Sandbox and synthetic login peer to exercise the actual restart POST and MCP management dispatch. It checks persistent Off, child exit, retained files, explicit On, and busy calls without touching everyday servers/accounts. The 3 cases pass; private Manager guards and actual widget checks are in their source modules.

## Approved application icon

native_app_icon.m runs in a disposable AppKit process with no windows or Fetch state. It checks the real bridge setter,72DPI/1024pt image size, close256px raster match, and queued assignment after caller bytes are overwritten/released. Uses a distinct sentinel and main-queue marker; AppKit may reuse getter objects. No account/Keychain/backend/MCP access. Evidence/commands live under milestones/app-icon.

## November 11 — remove Retina outline, fix all native hover and align resize

native_window_controls.m now compares all three buttons separately and exact exit restoration, and checks native frame/content edges match. This closes the older any-group-change assertion that could miss red/yellow hover failures. Uses a hidden disposable window only.

## November 11, 2026 — expansion, refresh hover, left window controls and right shadow

native_window_controls.m compiles with AppKit/LocalAuthentication/Security and checks a hidden disposable window using the actual bridge: top-left anchors, resize, group-hover rendering/state and close delegate routing. It does not launch Fetch or use account/state credentials.

## November 6 — Codex initialization offer

local_mvp_acceptance.rs adds one realistic client flow: offer2025-06-18without a negotiated HTTP version header, receive Fetch's supported2025-11-25, send initialized with that header/session, then list the permitted Fetch tool. It failed with400before the negotiation correction and now passes along with12existing tests in that target. Injected authority/disposable files only; actual saved-client reconnect remains distinct.

## November 6 — browser approval form

The focused in-process flow lives in src/oauth/tests.rs. It registers a disposable client, loads the real approval HTML/cookie through the full router, checks its browser-compatible referrer policy, rejects null/foreign Origin submissions, approves via the existing test-only native fixture, and exchanges the PKCE-bound code for usable Fetch access. Disposable encrypted files/fake keys only; no real client, Keychain, browser or Mac approval. The test fails against the original no-referrer response and passes after correction.

## Python MCP installation — November 4

`python_installation.rs` submits imported Blender and Time definitions through
the actual Installer preparation path. A fake uv runs inside the existing
deny-default macOS sandbox after owned-file/outside-file/sibling/network canaries
pass. It checks normal uv environment installation without a forced Python,
explicit Python and package-version preservation, server arguments, resolved
metadata/executables/helper folders and separate private installation homes.
It never starts either MCP or reads personal accounts/configurations.

## Figma Remote unsupported import — November 4

Two focused fixtures in import_compatibility.rs/direct_install.rs cover remote URL/recognized bridge rejection, its reason and unselected status; Desktop/other imports remain capturable, source bytes unchanged, direct fresh install creates no saved server/package. Existing plus new tests pass19/19; no provider, MCP process or credentials are used. Native build passes5.38s and independent review ACCEPTs; evidence under milestones/figma-remote-unsupported/.

## Selected Codex discovery — November 4

Existing HTTP fixtures in backend.rs now require one Codex source when `.codex` is selected despite neighboring Claude/OpenCode configs. Home/default discovery keeps its existing expectations. Installed metadata is checked on the sole selected Codex source; home partial-failure coverage remains. The selected-folder test reproduced the original bug before the source fix (three groups instead of one). Final focused verification passes 10/10 HTTP discovery tests and the native build; see milestones/selected-discovery-scope/info.md.

## Scrolled-parent terminal pointer — November4

ui/server_terminal_tests.rs now drives clicks through an actual scrolled Iced page. It reproduces raw-window versus adjusted-cursor selection jumps, checks small normal movement, intentional bounded drag and click-to-clear through rendered pixels.11terminalUI checks1new/native build pass; prior direct-widget tests lacked this parent transform. No native app or credentials. See milestones/terminal-pointer-coordinates/evidence.md.

## Terminal selection/trackpad interaction — November4

Two actual-widget/headless scenarios in ui/server_terminal_tests.rs reproduce release-outside stuck selection and fractional trackpad capture failure before the fix. Final10terminalUI checks prove bounded selection pixels after re-entry, older-output visibility/up-down scrolling, empty-poll stability and existing rendering/session behavior. No native app, shell or credentials used. See milestones/terminal-selection-scroll/evidence.md.

## Normal command-search folders — November4

The focused native_storage fixture uses a disposable child PATH and a real confined PTY fed by the shared transport builder. Bare fake-global gh resolves by name, then a private gh wins. No real gh or personal credentials run. Existing program_paths startup/setup fixtures expect the inherited default suffix and retain private-first checks.23focused checks and native build pass; see milestones/normal-program-path/evidence.md.

## Console/Terminal verification — November 4

New focused checks live in src/terminal/tests.rs (real deny-default confined PTY and native-only route requests), src/runner/log_store_tests.rs (private sanitized disk persistence/reload/bounds/unsafe-file handling), and src/ui/server_terminal_tests.rs (retained pane/model state, ordered/stale input, explicit close, fresh versus reused terminal queries). Existing service-log assertions follow the approved newest-output retention policy. Actual commands/outcomes and native appearance limits belong in milestones/server-console-terminal/evidence.md once final checks finish.

Continuous-console tests cover passive expanded state, preserved startup-to-auth history, collapse/window restore, manual-save state and new request errors beside old output. Manager checks cover bounded sanitized carryover and exit facts. The actual confined fixture uses initial Manager::startup before Authenticate without reading logs first. The older standard-auth clearing assertion now requires retained history once. See milestones/continuous-server-console/evidence.md for 150 passing relevant tests and the separate failed-Restart observation.

service_startup_logs.rs and native_state/service_log_peer.py cover actual confined output/errors/instructions/exit and disposableHTTP replies. Manager/SDK service_log_tests modules cover privacy/lifecycle/redaction/OAuth details; updatedUI tests require actual-output console and fallback cleanup. Final184uniquechecks15new,31UIrecheck/nativebuild pass; see milestones/service-startup-logs/evidence.md. No live account/window proof.

program_paths.rs verifies actual bare-name helper execution through ordinary startup and recorded setup in the existing deny-default sandbox. It checks legacy records without rewrites, other-installation negative lookup, paths with spaces and private metadata projection. No Railway package/account execution. See milestones/private-program-paths/evidence.md for the final independent results.

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

standard_authentication.rs tests the actual start-route request, retained auth
deprecation, local startup/helper link privacy, pre-initialize waits beyond30s,
last-known/current separation, missing inputs, active-call preservation and owned
helper Quit. It uses verified deny-default peers in native_state/login_peer.py.
Results are recorded separately in the standard-authenticate milestone.

November4 Codex startup-environment checks add eight synthetic-parent helper tests and one focused transport test; program_paths.rs now also asserts actual argv/cwd/HOME/XDG/PATH/LANG in confined normal/setup children. Scoped results:24integration +8standalone pure cases and native build pass, independent Evaluator ACCEPT. Full library execution awaits separate terminal test wiring. See milestones/codex-startup-environment/evidence.md.

## November 12 — final native resize and UI checks

native_window_controls.m now tests a six-point grip inside the visible rounded
window, exclusion of outside/transparent/center areas, native resize-direction
forwarding, untouched unconfigured windows, resizing, hover controls, close and
teardown. It uses task-owned hidden windows only. Actual user-window cursor/drag
feel remains a live check. New UI fixtures in neumorphic_tests.rs cover shared
copy feedback without backgrounds, white icon shine, instant manual reload/card
entrance, Connect ordering/marks and pending-start/immediate-result states.

## Server-first tools — November 12

`server_first_tools.rs` uses the actual gateway, API management adapter and runner
with disposable in-process HTTP peers and memory credentials. Four focused flows
cover startup server IDs without upstream tool loading, parallel selected-server
catalogs with stable ordering and unavailable origins, unchanged direct HTTP
defaults, generic native results and permission refusal, modern continuation
routing, and transactional stable alias persistence. It never starts installed
clients, native apps, provider logins or personal state. Root coordinates execution;
fixture source alone is not evidence of a passing run.


Workspace native conversation acceptance uses installed official helpers on disposable transcripts via actual native routes; it checks full long replies, named tools, project authority, unchanged source/raw reads and successful snapshot cleanup. The ordinary caller case covers conversation list/read and Finder reveal too. No real Finder launches or personal histories are used.
