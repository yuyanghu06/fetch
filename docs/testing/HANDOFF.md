# Fetch MVP — compiled, operational work and testing remain

D81 current local work: public import compatibility and shared native/browser
rendering are implemented; generic remote discovery/registration/renewal source
is added with issuer-bound callback protection. The final locked native binary
build succeeds and68 selected fixture checks pass together. Independent source
review ACCEPT is limited to that scope. Credential-outcome and authenticated
remote-verification choices remain pending; everyday-provider setup is not
universally complete. A fresh confined offline Codex starts with zero MCPs, but
actual UI installation/provider/native/client acceptance has not run. Mac test
user setup currently requires administrator authentication. No packaging/signing/
DMG. See [LOCAL_MVP_RUN.md](LOCAL_MVP_RUN.md); older no-execution statements below
describe earlier passes.

Current October29 status: five implementation areas plus runtime acceptance and
distribution remain in [MVP_GAPS.md](MVP_GAPS.md). Railway authentication and safe
request logging source are implemented and compiled (latest source build4.66s);
actual sign-in/native/client checks remain unrun. The full prior working tree,
including Figma documentation, was pushed as a15eacc. Earlier D77/D79 build
paragraphs below are historical evidence, not full-MVP completion.

Historical October28 source/compile evidence: D77 delivered initial integrated source without execution; later audits found the remaining gaps listed above. The user subsequently authorized dependency resolution and binary compilation/build fixes under **D79**. The native development binary now compiles successfully. **Agents ran no tests/test targets, app/service launch, MCP/provider connection, native/Keychain/trust action, managed server setup or packaging in those source/compile passes.** The user controls all remaining execution. Earlier foundation results apply only to their recorded commits.

## What is prepared

| Requirement | Source |
| --- | --- |
| Account-free app connection, scoped tokens, rotation/revocation | `src/oauth/`, `src/authority.rs` |
| Native Settings authority, fresh Mac confirmation and Keychain keys | `src/native_auth/`, `src/credentials/` |
| Stable local HTTPS identity and one app instance | `src/identity/`, `src/application.rs` |
| Safe discovery/source capture and private pinned installation/progress | `src/discovery/`, `src/installation/`, `src/storage/` |
| Config, credentials, provider authentication, named restart/reconnect | `src/manager/`, `src/api/runtime.rs` |
| Stdio, Streamable HTTP and legacy SSE; raw tools/arguments/results | `src/runner/` |
| Native MCP revisions, management tools, progress/owned interactions, resources/prompts/completion | `src/mcp/` |
| White native interface, selection/install/status, expandable inspection, call settings, private secrets and client guidance | `src/ui/` |
| App bundle/drag-and-drop DMG script and source license | `packaging/`, `scripts/package-macos.sh`, `LICENSE` |

The interface calls the same HTTPS application APIs as agents. It uses structured states, not next_steps text, to choose controls. A status read never starts a login or server. A configuration save never implicitly restarts. Accepted installation/restart responses remain pending until their named operation finishes.

## Compilation completed; the binary was not run

The first native build refreshed Cargo.lock and exposed one gateway match-guard syntax error. After correcting that syntax and unused imports, the exact command below succeeded with exit0 in12.61seconds on Rust/Cargo1.95.0:

```sh
cargo build --locked --bin fetch --message-format short
```

The build produced `target/debug/fetch`; it was **not executed**. The resolved lockfile is now saved for repeatable binary builds. Dependency resolution used the manifest's Rust1.89 compatibility constraint; this pass did not separately build with Rust1.89 or other targets/profiles.

Remaining warnings are four deprecated classic-Keychain API uses in the native bridge and one unread protocol-error field. They do not prevent compilation. The selected Keychain behavior was preserved; no storage redesign or native function call was performed to remove warnings.

Compilation does not prove runtime behavior. Subsequent formatting/linting, tests, HTTP checks, native behavior and client/server connections remain separate actions requiring the user's instruction. No CI workflow or Git hook was added to start them on push; commits and pushes use hooks disabled.

## Before launching or connecting anything

Finish the protection and account setup described in [the isolation plan](../architecture/isolated-testing.md). The proposed Fetch-Testing folder is not an approved/provisioned isolation boundary. Merely setting a data directory or a different client profile does not isolate Keychain, desktop apps or services. Use disposable fixtures and dedicated credentials, and retain personal clients/configurations untouched.

The native `fetch app` command, when you choose to run it, creates protected installation state and Keychain items, starts its HTTPS listener and starts saved managed connections. Native debug builds use `~/Library/Application Support/Fetch-Development/`; release builds use `~/Library/Application Support/Fetch/`, resolved from the current home. Explicit `--data-dir` and `--discovery-home` options select separate fixture roots, and `--port` selects its localhost port. An explicit `--discovery-home` disables inheritance of the everyday `CODEX_HOME`; its built-in Codex candidate stays under that fixture home. Those options alone do not establish the required isolation. A second launch requests the first instance's window instead of starting duplicate managed servers.

The original `fetch dev` command remains fixture-only and ephemeral, with app authority denied. It is not the full native MVP launcher.

The native debug endpoint is `https://localhost:53672/mcp`; release uses
`https://localhost:53671/mcp`, preserving explicit overrides. The native UI trusts only its own loaded certificate for its internal API connection. External browsers and coding apps require the selected explicit certificate trust setup and normal verification. No certificate trust or personal client changes were made, and no insecure TLS workaround is prescribed.

## User-controlled verification order

1. **D79 completed:** resolve dependencies and compile the native binary; save the resolved lockfile and compiler fixes. No runtime acceptance follows from compilation alone. Further execution remains paused.
2. Choose and prepare the actual isolated Mac boundary, disposable data and dedicated accounts before any OS/provider/client exercise.
3. Run local record/JSON/authority/HTTP checks, then native approval/Settings/Keychain/TLS and lifecycle checks. Confirm ordinary agents cannot write manual secret values.
4. Use [the curated roster](../../milestones/isolated-mcp-testing/servers.md): Everything, Filesystem, Time, Tavily and Linear. All remain **not run**. Cover preparation, required authentication, one harmless tool operation and saved context reuse; keep this first batch short.
5. Check queues, raw JSON/media, both native revisions, resources/prompts, owner-bound progress/interactions, cancellation/uncertainty, named restart/reconnect and revoked access. Use the existing selected contracts as expected behavior.
6. Exercise Claude Code, OpenCode and Codex and compare native controls with guidance for the same states. Do not clean personal configurations until successful verification and explicit user action.

This is a future sequence, not permission to start it automatically.

## Boundaries to examine first

- The Rust/Objective-C/Iced integration compiles. Fresh Mac approval, Keychain ACL/build identity behavior, Dock/menu/window reopening, listener shutdown and signed/ad-hoc build behavior all need actual evidence.
- Supported imports are conservative. Unknown launch argument grammars remain unsupported. Remote records support known non-secret endpoint path profiles (`/mcp`, `/sse`, `/api/mcp`, plus Linear `/mcp/readonly` and the exact HTTPS Railway root on port443, with permitted trailing slash); custom layouts, credential-bearing/query/fragment URLs remain unsupported instead of copying possible credentials.
- Credential verification adapters cover documented Tavily/GitHub reads, Linear's authenticated viewer read and Railway's stored-token /oauth/me identity check. Railway has Fetch-owned DCR/PKCE and D80's temporary numeric-loopback callback; its actual flow remains unrun. Linear OAuth metadata/PKCE/token ownership/refresh is implemented, but its issued token must actually pass the selected service check. A browser approval or successful MCP handshake cannot substitute. Unknown/wrapper-owned integrations report their missing support truthfully.
- D31 has no unchecked-success exemption. Unsupported, unreferenced, multi-entry, removal-only or OAuth-shadowed manual changes may already be saved but return `CREDENTIAL_VERIFICATION_FAILED`. This boundary is documented and must be checked; no old token's success can verify a new manual value.
- Host handling of aliases, encoded selectors/resource templates, large/unknown JSON values, modern protocol headers/control messages and raw media is unverified. Deferred native Tasks and live resource subscriptions must remain unadvertised.
- Documented Codex/OpenCode call-wait preferences initialize Fetch defaults; the supported observed-limit branch stays empty unless a trusted integration can supply one. Verify default/override/allowance and dispatch-relative timing without changing raw arguments.
- New numeric limits are source engineering choices recorded in [the implementation brief](../architecture/mvp-implementation.md). They need compatibility evidence. All source-review acceptance here is bounded reasoning, not proof of runtime behavior.

## Packaging remains explicit

`scripts/package-macos.sh` takes an already-built binary. It requires `FETCH_ALLOW_PACKAGING=1` and a fresh output folder, and never starts a build or test. It can apply an explicitly supplied signing identity. It does not notarize, publish or create a GitHub release. No app bundle or DMG has been produced.

Before distributing, collect the exact resolved direct/transitive license texts and runtime notices, and complete authorized signing/notarization and acceptance. [The notices file](../../THIRD_PARTY_NOTICES.md) is a source-stage inventory, not a complete final binary notice bundle.

## Intentionally outside this MVP

Custom GitHub CLI tools are deferred by D78. Shared memory, PostgreSQL/retention, graphs, extra managed processes, native background Tasks, live resource subscriptions and future project/session sharing remain deferred at their recorded scopes. Existing GitHub MCP imports remain separate selected scope. No release is authorized.

## Delivered source commits

The scoped source increments are bf5dd73 (authority), fc9b1ae (installation), 9fc4eee (runner/native MCP), ff1f07c (management), 3024639 (native/API integration) and e576bcc (timeout/input refinements). They were pushed to origin/main during implementation. The final documentation/packaging-source commit accompanies this handoff. Source-only reviews accepted their bounded scopes after corrections. No current-pass runtime verification exists. D79 adds binary compilation evidence only.


## October 28 correction — operational gaps and auth work

The user reaffirmed a fully working selected MVP. The earlier source inventory
is not a claim that all required operational paths are complete. Read
[MVP_GAPS.md](MVP_GAPS.md). Inline authentication/progress and safe request
diagnostics source are now added. Railway DCR/native browser callback and
private CLI preparation are implemented under D80 and compile; no live sign-in or
provider/MCP connection has been performed. See [diagnostics](DIAGNOSTICS.md).
