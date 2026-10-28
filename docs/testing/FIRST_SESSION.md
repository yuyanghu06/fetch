# First testing session

October 28, 2026. This turns the existing short-test requirements into a proposed
execution sequence. Planning is active. No agent ran a build, test, app or MCP
connection during this pass. The user reported running the development build;
the agents have not observed a successful launch or native window.

## Existing requirements

Keep checks short: installation, authentication, ordinary MCP calls and saved
login reuse. Preserve the selected Everything, Filesystem, Time, Tavily and Linear
roster. Use disposable files, separate client state and dedicated test credentials.
Protect everyday files, credentials, client settings, apps and services. A separate
data folder alone is not isolation. Heavy workflows and desktop-app control are
outside this first batch; broader MVP acceptance remains pending.

## Current user-selected connection flow

The user now wants Fetch to import supported normal global Codex definitions,
then a fresh test Codex profile whose only MCP connection is Fetch. The main
Codex client can connect to the same running development Fetch instance. Follow
[DEVELOPMENT.md](DEVELOPMENT.md) for portable paths, separate development state,
shared-backend behavior and setup prerequisites. The sequence below retains the
broader short-test checks; a second Fetch instance is not needed just to test a
fresh Codex client login.

## Native development command

```sh
cargo run --locked --bin fetch -- app
```

This compiles if needed and opens the native app with debug Fetch data under
`~/Library/Application Support/Fetch-Development/` on port 53672. Startup creates/loads Keychain-backed
identity, starts its local HTTPS listener and starts saved connections. The `dev`
subcommand runs the older fixture backend without the native window. A Git clone
does not bring the previous environment's compiled binary.

The user's first attempt failed because the Apple toolchain required Xcode license
acceptance. They reported accepting it, then running the development build. That
error alone indicated no necessary Fetch source fix. Do not accept legal terms on
the user's behalf or automatically launch another instance.

## Proposed checks and expected results

1. **Existing automated tests.** When the user starts automated execution, run:

   ```sh
   cargo test --locked --lib --test backend --test raw_json
   cargo test --locked --test development_https
   ```

   Source inspection finds 30 tests in the first group and seven in the second.
   The four new environment-path tests are a separate, unexecuted target:
   `cargo test --locked --test environment_paths`.
   Record actual counts/results. They use temporary files, fake keys and test
   authority. The HTTPS group opens disposable loopback listeners; certificate
   trust stays inside its test clients. Neither group opens the native window,
   uses production Keychain, installs MCP servers or contacts providers. Cargo
   may download dependencies and compiles/links the native bridge.

2. **Empty native app.** Establish the actual native-test boundary first. Retain
   the already documented proposal of a separate standard macOS test user for
   native/Keychain work, with file/service access checked; this remains a proposal,
   not a provisioned account. Choose explicit absolute test data/discovery paths
   and an available port, with no personal server definitions. These paths alone
   do not isolate Keychain or process access. Expect a usable empty window, stable
   certificate across reopening, a second launch reopening the existing window,
   red-close keeping the backend alive and explicit Quit stopping it. Record what
   happens rather than inferring success from compilation.

3. **Everything first.** Install through Fetch using a deliberately clean server
   environment. Initialize/list tools, echo a marker and calculate 2 + 3 using
   actual advertised schemas. Expect the marker unchanged and result 5. Check
   one structured result and one resource/prompt. Restart and repeat a small call.
   Stop and fix a failure before adding another server.

4. **Filesystem and Time.** Verify effective allowed directories, write/read a
   marker inside a disposable root and expect refusal for a disposable canary
   outside it. Convert UTC 12:00 to Tokyo 21:00 and check an invalid-timezone error.
   This covers both Node and Python installation paths without provider accounts.

5. **Tavily and Linear.** Use dedicated credentials/accounts. Tavily's newly
   saved/retrieved key must pass authenticated `/usage` verification; a keyless
   search is not authentication evidence. Check an invalid key, restore the valid
   test key and verify reuse after restart. For Linear, cancel consent, retry and
   approve, read one test issue through the read-only endpoint, then check saved
   authorization reuse. Do not claim token expiry/refresh without observing it.

For each check record exact versions, expected/actual outcome, a redacted failure
and cleanup needs. Review package availability/integrity before installing roster
pins. Keep raw logs/tokens/browser state outside Git. Cleanup includes Keychain
and provider tokens as well as files; deleting a folder alone is insufficient.

Run formatting and strict lint separately from the first behavioral tests:
`scripts/verify-dev.sh` runs formatting, strict Clippy, then tests and stops at the
first failure. Recorded warnings and visibly unformatted new source may prevent
reaching tests; this is a source-based concern, not a current check result.

Later acceptance still includes HTTP/SSE, both MCP revisions, real Claude Code,
OpenCode and Codex workflows, authorization/revocation, Settings-only secrets,
queues/deadlines/cancellation/restart and interaction ownership. This first batch
does not complete those requirements or authorize a release.
