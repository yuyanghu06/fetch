service_log_peer.py is a synthetic startup peer for service_startup_logs.rs. It emits known stdout/stderr/instructions/errors under the existing verified deny-default confinement, never using a provider/account. Assertions cover retained80lineprefix, returned-32001reason, helperexit9, invalidstdoutfailure and no successfulframe logging.

program_paths.rs reuses confinement.rs to test installed helper lookup with an embedded synthetic peer. A helper symlink targets the already permitted public CLT Python runtime; outside-file, sibling-state and network canaries run before each child test. No additional app or real provider access is granted.

# Confined native state fixtures

The synthetic Python MCP peer owns ordinary files under its HOME and XDG folders.
It has no token interception, secret bridge or provider implementation. Each peer
runs inside a separate macOS deny-default sandbox. Disposable read/write/network
and sibling-state canaries must fail before MCP execution; owned reads/writes must
succeed. Python and its standard library are read-only public CLT runtime files.

These fixtures verify native-file reuse, instance separation, raw MCP forwarding,
other-server responsiveness and preserved state on removal. They do not establish
real provider login, native Keychain, Finder or universal server compatibility.
All temporary state and owned processes are removed after each run.

login_peer.py is a disposable sign-in fixture for standard_authentication.rs. It
prints a synthetic login link before initialize and waits for test-owned files;
helper mode also creates an owned child to verify Quit reaping. It runs only via
confinement.rs's deny-default launcher after canaries, never with real accounts.

November4 program_paths.rs reuses these deny-default Sandbox canaries to verify the Codex environment adaptation in real disposable normal/setup children. No new provider package or account is used. Actual child args/cwd/private HOME/XDG/helper PATH/LANG are checked; cleanup removes task roots.
