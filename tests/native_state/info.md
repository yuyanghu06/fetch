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
