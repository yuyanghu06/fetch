# MCP MVP source implementation

## Goal and scope

Write the D73 selected Rust/macOS MCP first release through a user-controlled testing handoff. D77 prohibits every testing/build/check/launch/connection/package/packaging action in this pass and authorizes scoped source commits/pushes. D78 defers custom GitHub CLI tools.

## Completion criteria

Selected source paths and distribution scripts are written/integrated; bounded source reviews are addressed; scoped commits are pushed; source limitations and first verification steps are recorded. This milestone measures source delivery only. Runtime and release acceptance stay separate.

## Status

Source work is complete through the handoff. Scoped source increments through e576bcc are pushed to origin/main; final handoff/distribution-source documentation accompanies this completion record. Authority, installer, manager, runner/native MCP, HTTP and native UI source are integrated. Packaging is source only. Read [the handoff](../../docs/testing/HANDOFF.md) and [current snapshot](../milestone.md).

The D77 source pass ran no compilation/dependency resolution/lockfile refresh, tests, formatters/linters/validators, app/service/native/Keychain/trust action, package/runtime execution, provider/MCP/client connection or packaging was run. First compilation may require source fixes. Source-review ACCEPT is not tested acceptance. Actual supported-client/provider behavior is unverified.

The user chooses when to start verification and prepare the isolated environment/accounts. Personal state is unchanged; no release or DMG has been created. Prior pass details remain in append-only history.

D79 subsequently authorized compile-only fixes. The native binary now compiles; no tests, app launch or MCP connection occurred. See [the compilation milestone](../mcp-mvp-compilation/info.md).
