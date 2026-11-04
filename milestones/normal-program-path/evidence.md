# Normal command-search PATH — verification

November4,2026. Independent Evaluator ACCEPT;23 focused checks pass,1new feature test; locked native build and whitespace pass.

## Final source

src/manager/native_state.rs applies the existing inherited-environment builder before composing PATH. Private installed folders remain first. Explicit assigned PATH keeps the existing private+assigned behavior. Otherwise the executable parent precedes inherited PATH; existing /usr/bin,/bin fallback applies only when inherited PATH is absent. Normal MCP startup, recorded setup and Terminal already share this builder. No other production source change in this pass; private HOME/XDG, arguments/cwd and service credential handling remain.

## Checks actually run

- `cargo test --locked --lib manager::native_storage_tests`:13passed. One new realistic feature test runs a disposable child process with a fake host PATH, passes the shared transport environment into an actual deny-default confined PTY shell, types bare gh and observes the fake global executable result, then observes a private gh take priority after shell command-cache reset. Existing explicit assigned PATH and private account checks pass.
- `cargo test --locked --lib manager::process_environment_tests`:8passed. Existing inheritance/assignment/certificate/internal-variable checks unchanged.
- `cargo test --locked --test program_paths`:2passed. Real confined normal MCP startup and recorded setup still find private helpers with inherited default PATH retained.
- `cargo build --locked --bin fetch`:PASS8.40s; existing warnings remain.
- Scoped and final `git diff --check`:PASS.

Root and Evaluator inspected original output at /private/tmp/fetch-inherited-path-native-storage.log, fetch-inherited-path-environment.log, fetch-inherited-path-programs.log and fetch-inherited-path-build.log. One new test plus existing22checks; no full-suite claim or duplicate count for the child invocation.

## Resource and acceptance limits

Fake gh files live only in a disposable fixture; no real gh command, personal account/token/Keychain or provider connection was used. Child-only PATH assignment avoids mutating parallel test environment. Sandbox confines actual shell files/processes; owned PTY shuts down and0task fixture folders remain. No existing native app/service stopped or relaunched by agent.

This inherits PATH already available to Fetch. It does not load shell profiles or find folders absent from that PATH. The current coding/development shell includes Homebrew. Native user terminal and real account login still need observation after explicitly quitting Fetch, running ./scripts/run-development.sh and opening a fresh terminal. Existing terminal processes retain their original environment until closed/restarted.
