# Direct installation body verification

November3,2026. Backend-only extension; no new UI. Independent Tester/Evaluator
ACCEPT; no material findings.

All commands used process-scoped
`DEVELOPER_DIR=/Library/Developer/CommandLineTools` without changing Mac settings.

| Command | Actual result |
| --- | --- |
| `cargo test --locked --lib installation::direct_tests` |6 passed; compile11.39s/tests0.07s. |
| `cargo test --locked --test direct_install` |8 passed; compile1.34s/tests0.11s. |
| `cargo test --locked --lib --test backend --test import_compatibility --test local_mvp_acceptance --test raw_json -- --skip installation::direct_tests` |128 existing checks pass: lib93,backend16,imports7,MVP11,raw1; compile1.79s. New6 excluded to avoid repeating already passing checks. |
| `cargo build --locked --bin fetch --message-format short` |Exit0,0.35s. Existing five Security bridge deprecations and unread MCP error-field warning remain. |

142 unique checks passed; no failures or production fixes. New14 fixtures cover
shared npm/uvx/script/bridge capture, quoted argument/normalized identity,
unsupported shell/credential input, direct URL preparation, malformed/duplicate
requests, existing source identities, preserved saved edits/encrypted fixture
credentials, new IDs for changed names/definitions, sanitized progress and safe
partial failures. HTTP tests cover management authorization,202/Location/progress,
400 validation and no-store. Stub connection hooks verify sequential preparation,
active-job409 and runtime/auth failures preserving preparation success.

All inputs/files/keys are disposable. Pure command tests never install packages;
preparation-only integration uses saved remote definitions/inert files without
Connect. Connection-hook outcomes are simulated, not live-provider evidence.
No package download/install, managed executable, MCP/server/provider/helper,
native prompt/Keychain/personal config operation was run. This does not establish
real end-to-end package installation, login or client compatibility.

Exact already-locked shlex2.0.1 is used as a direct parser dependency with no
additional runtime dependency or copied upstream source. Config imports and
private preparation/Connect rules remain. Pre-existing unrelated UI/startup edits
were preserved and excluded from this feature.

Source/evidence delivered as53adad6; live remote/local refs matched. Unrelated
UI/startup/deletion-investigation and older documentation edits were preserved.
