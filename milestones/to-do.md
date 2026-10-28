# Fetch current work

October 28, 2026. D77 source implementation is written. D79 now authorizes binary compilation/build fixes and commit/push only; tests, app launch and MCP connections remain prohibited. Earlier unfinished tasks are preserved in [the prior rolling record](to-do-before-d77-source.md); source implementation replaces its old implementation waits, while all unrun verification remains pending.

## Active compile-only pass

- [x] Inspect Planner scope and native build script; no compile-time app action.
- [x] Resolve dependencies/refresh Cargo.lock through first binary build.
- [x] Fix gateway syntax/imports; locked native binary compilation succeeds without tests or launch.
- [x] Locked native binary build succeeds; independent source/build-evidence review ACCEPT.
- [ ] Commit/push fixes and refreshed lockfile; record outcome.

## Source work

- [x] D71 OAuth/native app authority, rotating tokens/revocation, Keychain-backed file keys and lasting HTTPS identity source.
- [x] Conservative discovery/private source capture, pinned Node/Python/uv preparation, isolated server packages and installation progress source.
- [x] Scoped server reads/config/manual credentials/authentication/owned refresh/named restart-reconnect and saved-versus-applied state source.
- [x] Stdio/Streamable HTTP/SSE runner, raw data/catalogs/aliases, queue/deadline/uncertainty/cancellation and selected native methods source.
- [x] Existing application API adapters, matching management tools and native Iced interface/lifecycle source.
- [x] App/DMG packaging source, source MIT license and provisional dependency notice inventory.
- [x] Bounded independent source reviews; integrate reported source corrections. These are not tests or runtime acceptance.
- [x] D78 defer custom GitHub CLI tools; preserve existing GitHub MCP imports.
- [x] Push scoped source feature commits through e576bcc; final handoff/distribution-source documentation is a separate completion commit.
- [x] Write [the explicit testing handoff](../docs/testing/HANDOFF.md) and preserve personal state.

## Deliberately unstarted — user controls all execution

- [ ] Complete D79 compile-only work above. Runtime acceptance remains unstarted.
- [ ] Start any formatting/lint/test/HTTP/validation command only when the user chooses. Nothing is recorded as passing in this source pass.
- [ ] Select/prepare the actual file/Keychain/app/service/network isolation and dedicated test accounts; proposed layout remains unprovisioned. VM/provider selection is not forced; D46 protection still applies.
- [ ] Run the curated short roster: Everything2.0.1, Filesystem2.0.1, Time2026.10.19, Tavily0.2.22 and hosted Linear. All installation/auth/tool/reuse checks remain NOT RUN.
- [ ] Establish actual native approval/Settings/Keychain/encryption/TLS and window/background/Quit/single-instance behavior.
- [ ] Verify API/server lifecycle, raw transport fidelity, both revisions, resources/prompts/relay, visibility/ownership, queues/deadlines/cancellation/uncertainty/restart/revocation and matching UI/guidance.
- [ ] Verify Claude Code/OpenCode/Codex workflows and fresh installation with private runtimes. Broader D58 acceptance is preserved beyond the first short batch.
- [ ] Resolve real findings and obtain independent runtime/evaluator acceptance; do not treat source review or a build as sufficient.

## Distribution and future scope

- [ ] After dependency resolution, collect exact resolved direct/transitive/runtime license texts before binary distribution.
- [ ] Explicitly build/package/sign/notarize as selected and authorized; packaging script was not run and no app/DMG exists. No release/publication is authorized.
- [ ] Keep personal client cleanup manual and after successful verification; never perform it automatically.
- [ ] Preserve future project/session organizer, common cross-computer context/MCP sharing, same-product memory, PostgreSQL/retention, graph, extra processes and deferred native features for a later selected scope. Their design/implementation is deferred, not completed or cancelled.
