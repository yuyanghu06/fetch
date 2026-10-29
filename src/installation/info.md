# Server preparation

`mod.rs` captures selected source entries without retaining credential values. It keeps one installation group, prepares entries sequentially, reuses saved successful imports and retains progress for the current run. Packages and runtimes belong under Fetch's private Application Support root; durable server records remain separate.

Source includes pinned Node 24.22.1, uv 0.13.5 and Python 3.13.18 provisioning, HTTPS download limits, published SHA-256 comparison, private preparation homes/caches and isolated npm/Python environments. No runtime setup or package command was executed during implementation. Fresh-install behavior and all failure/cleanup paths await user-controlled testing.

Primary references: [Node distribution](https://nodejs.org/download/release/latest-v24.x/), [Python release](https://blog.python.org/), [uv release](https://github.com/astral-sh/uv/releases/tag/0.13.5), [uv storage](https://docs.astral.sh/uv/reference/storage/) and [managed Python](https://docs.astral.sh/uv/guides/install-python/). No third-party implementation was copied.

The injected root now defaults to ~/Library/Application Support/Fetch-Development
for native debug builds and ~/Library/Application Support/Fetch for release.
packages/servers, packages/runtimes and packages/cache stay below that root;
existing local scripts remain in place and remote services remain remote.
No installation was performed to verify these paths in this change.

Official @railway/mcp-server imports also prepare pinned @railway/cli5.71.0 in
a separate railway-cli dependency folder under their private installation. This
provides the executable required by the MCP shim without using personal PATH or
modifying the shim's installed dependency tree during repair. Existing installs
repair it only on explicit Connect, never GET or automatic startup. Native binary
existence/permissions/path containment and pinned package metadata gate readiness;
no CLI --version is run. npm's official postinstall fetches its binary over HTTPS;
it does not provide an independent archive checksum. No package install executed
in this source pass. First-party download HTTP now uses safe diagnostics.
