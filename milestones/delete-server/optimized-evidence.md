# Deletion speed fix — November 3, 2026

User approved batching disk flushes after the baseline investigation. Implemented
in src/storage/mod.rs: remove_entry calls a private recursive helper, then flushes
the held containing directory once. The helper keeps every existing descriptor,
name, depth, ownership, inode/device and no-follow check. Missing-target retries
still reach the final flush. Credential/config writes and final server-record
removal retain their own durable flushes. No API or UI contract changed.

## Actual before/after

Independent Tester repeated the same exact ignored diagnostic cases in fresh,
verified macOS sandboxes, using copied public Node/npm and disposable fixture
keys. Baseline and optimized versions/entry counts match.

| MCP | Files / folders | Before | After | Time saved |
| --- | --- | ---: | ---: | ---: |
| Google1.11.3 | 13,871 / 1,798 | 75.225s | 0.889s | 74.336s |
| Everything2026.8.31 | 3,870 / 611 | 20.400s | 0.262s | 20.138s |

Google preparation took20.717s and deliberately bypassed Fetch Connect/login.
Package cleanup finished842ms after delete start; remaining fixture credential,
private/config/runner cleanup took47ms. This measures package deletion without
Google-account access, not working Google authentication.

Everything installed, connected, exposed13 tools and returned the real echo
marker in3.800s. Its process was gone by137ms after delete start (100ms polling);
stop plus package cleanup finished238ms and final cleanup took24ms.

Both selected tests exited0 with1passed/1filtered. Each verified package,
configuration and runner removal, while preserving source configuration, shared
runtime and unrelated marker. These are observed individual runs; future timing
varies with package contents and disk load.

## Safety, compilation and review

Independent storage checks passed6/6: four existing tests and two new containment/
private-state tests. New cases cover ordinary nested package modes, external
file/directory symlinks, sibling packages, shared runtime, missing-package retry,
config preservation and repeated private cleanup. Command:

```sh
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test --locked --lib storage:: -- --nocapture
```

Exit0; compile21.04s including artifact-lock wait; tests0.10s;104filtered.

Native binary compilation passed in13.52s:

```sh
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --locked --bin fetch --message-format short
```

An initial build caught an unfinished concurrent helper-auth edit outside this
change. Its owner completed the function; the retry passed without changing
those files here. Existing native deprecation/unread MCP-field warnings remain.
Independent source and final evidence evaluation ACCEPT the scoped change.

Sandbox probes permitted owned file writes and child execution, and denied
outside file reads/writes, signals to an outside disposable process, outside
loopback connections and securityd lookup. Registry HTTPS succeeded. Filesystem
metadata reads were allowed for DNS; fixture keys avoided real Keychain use.

After evidence retention, both disposable roots, copied runtimes, packages,
caches, outside canary and pointer were removed. All four owned paths were
verified absent, no owned process remains, and existing everyday Fetch/Node/
Railway processes stayed running. Personal settings/accounts were untouched.

The existing Fetch process was preserved. Quit and reopen Fetch to run the new
binary. Native window/HTTP/real Keychain/Google-account acceptance remains
separate; no new result is claimed for those workflows.

## Retained evidence

- [Google optimized events](optimized-google-events.txt)
- [Everything optimized events](optimized-everything-events.txt)
- [Probe, process, test and build results](optimized-evidence.json)
- [Baseline investigation](latency-evidence.md)
