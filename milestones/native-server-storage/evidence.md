# Native server storage verification

November3,2026. Evidence is bounded to the checks described here. No personal
credentials/configs, real Keychain/Finder/owner prompt or provider account used.

## Initial integrated checks

141 relevant checks passed before final review. The locked native binary built
successfully in25.89seconds. Final review found recovery/race issues; those initial
results are retained as history and do not establish acceptance of later fixes.

## Real unmodified server in resource confinement

Independent Tester staged official Node24.22.1 and original
@modelcontextprotocol/server-everything2026.8.31 in an explicitly marked task root
under /private/tmp, outside the repository and Fetch state. Node archive SHA256:
89c682593d76cd44233fa53b1563264e7de9cbb189cad169135c5b4fadfbc0a7,
verified against the official nodejs SHASUMS file. Package preparation used npm
--ignore-scripts with a restricted package-fetch sandbox and separate empty configs.

Runtime used deny-default macOS sandbox-exec with only owned file/runtime/system
library access and no network. Before executing the MCP package, positive owned
file and negative outside read/write plus loopback TCP canaries passed. No desktop
apps or account credentials were available to this no-account server.

The opt-in real_native_mcp test passed1/1:13 tools, echo before and after explicit
restart, registration removal, preserved native/legacy/package bytes and confirmed
owned child exit. Task root, Node,108 packages, npm cache/config/logs and temporary
state were removed; Tester verified no remaining task-prefix directories. The
fixture itself does not install packages and is ignored without an explicit root.

## Final review and fixes

Independent Evaluator initially requested changes for rejected saved access that
could not start a fresh login, stale rejection status after renewal, and expired
access captured before reconnect drain. Writers fixed those. Further review removed
GET writeback of stale derived status, correlated requests with the actual header
snapshot, and stopped treating bare permission-denied403 as proof of expired login.

Backend targeted8/8 checks pass. The new after-drain test uses an actual held raw
HTTP MCP call, Fetch's production self-HTTPS sender/certificate, SDK mock rotation
and reconnect HTTP fixture asserting the new bearer. Explicit login recovery uses
the SDK callback and a disposable browser seam. No provider-specific refresh code
was added. Final independent source review found no remaining material issue;
final integrated results and independent ACCEPT follow below.

## Final integrated run

Frozen production source command:

```sh
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test --locked --lib --test backend --test development_https --test environment_paths --test import_compatibility --test local_mvp_acceptance --test raw_json --test direct_install --test isolated_native_mcp --test real_native_mcp --message-format short
```

PASS exit0:151 passed,0failed,1ignored. Counts: lib84, backend24, development
HTTPS7, direct-install10, environment paths4, imports7, confined synthetic native2,
local MVP12 and raw1. The staged real package is intentionally ignored here; its
separate explicit1/1 result above makes152 checks exercised total. Compile15.79s.
The first final attempt exposed fixture queue capacity consumed by preserved
uncertain403/401 outcomes; the fixture now permits3slots and still asserts no
replay. Product uncertainty accounting was not changed to make the test pass.

```sh
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --locked --bin fetch --message-format short
```

PASS exit0, final incremental build0.47seconds. macOS Keychain API deprecation
and existing Rust dead-code warnings remain; no strict lint claim is made.
Independent final Evaluator returns **ACCEPT for this scoped refactor** after
source and test-log review. No production changes followed the final suite.
Tests/native logs are summarized here; temporary runtime/package roots were cleaned.

## Limits

Fake Keychain/native-confirmation connections and scripted OAuth responses do not
prove actual Mac OS permission/storage behavior, real provider consent/refresh,
Finder appearance or full coding-client compatibility. Native setups/folders need
recorded metadata; ordinary imports have no approved metadata authoring path yet.
Services without required standard discovery/issuer checks remain unsupported.
