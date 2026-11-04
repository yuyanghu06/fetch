# Real MCP deletion latency — November 3, 2026

User requested actual installation/deletion and an explanation of the delay, then
identified Google as the slow MCP. Both disposable measurements completed.
These are baseline measurements before the approved optimization. The fix and
latest timings are recorded in [optimized evidence](optimized-evidence.md).

## Boundary and method

Independent Tester executed the ignored test in tests/delete_latency_demo.rs
inside a macOS deny-default sandbox. Only task-owned temporary files and OS
runtime files were readable; writes were confined to the task root and /dev/null.
File metadata reads were allowed for DNS. External network access allowed package
installation; loopback access was denied. Positive/negative disposable probes
verified owned file writes/child execution and denied outside file reads/writes,
signaling an outside owned sleeper, connecting to an outside loopback listener,
and looking up com.apple.securityd.xpc. Outside probes succeeded; inside failures
were EPERM or BOOTSTRAP_NOT_PRIVILEGED1100. No real credential was read.

The test used real Installer, Runner and Manager with disposable fixture keys,
not MacKeychainKeys. A public Node/npm runtime was copied into the test cache.
Everyday Fetch, clients, credentials, configuration and services were untouched.
This is backend evidence, not a native window, HTTP, human approval, Google-account
or real Keychain acceptance result.

Two initial attempts failed during disposable setup: state directory mode0755
instead of0700, then denied /dev/null writes needed for ignored child stdio.
Both were corrected only in owned fixture setup. The third run passed; no product
fix was needed for those attempts.

## Google package installation/deletion

The user identified Google. A fresh confined environment prepared the same pinned
@a-bonus/google-docs-mcp1.11.3 through Installer::submit. Fetch did not invoke the
automatic Connect hook, authentication, catalog or tools; its runner stayed empty.
No Google sign-in or account credential was used. npm installation can execute
upstream lifecycle scripts; this is not a blanket assertion about their behavior.
The process audit observed no Google MCP startup.

| Observation | Result |
| --- | ---: |
| Package preparation | 21.487s |
| Package files | 13,871 |
| Package folders | 1,798 |
| Package cleanup finished (fixture-key stage starts) | 75.210s after delete start |
| Manager confirms deletion | 75.225s after delete start |
| Remaining credential/private/config/runner cleanup | 15ms |

This reproduces a roughly75-second delay with no Fetch-managed Google process
to stop. Samples showed steady package removal throughout, rather than a stalled
operation. Source definition, unrelated canary and runtime remained unchanged;
package, saved record and runner entry were absent afterward. Exact selected
test exit0:1passed,0failed. The complete [Google event log](latency-google.log)
and both runs' [confinement probe results](latency-confinement.json) are retained.

The15,669 package entries each triggered File::sync_all in the baseline recursive
removal. Together with the independent flush comparison below, this establishes
repeated disk flushes as the main cleanup overhead here. It does not measure a
production optimization or real Keychain removal time. Your existing Google
installation/account and running Fetch services were not part of either demo.

## Everything install/echo/delete

Pinned official @modelcontextprotocol/server-everything2026.8.31 installed through
Manager::install and auto-connected. Catalog contained13 tools; real echo returned
the disposable marker. Install plus echo took5.593seconds. Package contained3870
files and611 folders, without following symlink targets.

| Observation | Time after deletion started |
| --- | ---: |
| MCP process no longer present (100ms observer polling) | at most about139ms |
| Package still contains3666 files/600 folders | 1.021s |
| Package still contains1953 files/305 folders | 10.173s |
| Package still contains22 files/7 folders | 20.242s |
| Package removal finished, fixture-key deletion starts | 20.383s |
| Manager confirms deleted | 20.400s |

The last17ms covered fixture-key/credential-file, private-state, saved-record and
runner cleanup. Cumulative pre-key timing includes process close; the independent
process observer bounds that close near the beginning. The package cleanup
therefore accounts for nearly all the observed delay. Source/runtime/unrelated
canary remained unchanged; package, saved record and runner entry were removed.
Test exit0:1passed,0failed. Directory sampling adds some read overhead.

## Disk-flush comparison

After the real demo ended, a separate Rust std-only program created two disposable
folders of1000 tiny files. Both used the same unlink loop; one called
File::sync_all after every file, the other once at the end. Preparation was outside
the measured interval. This uses the same Rust flush primitive as Fetch.

| Deletion strategy | Total | Time inside sync_all | Calls |
| --- | ---: | ---: | ---: |
| Flush after every file | 4559ms | 4340ms | 1000 |
| Flush once after all files | 42ms | 4ms | 1 |

This demonstrates substantial repeated-flush overhead on this machine, not a
measured speedup of an implemented product fix. The baseline implementation flushed after
**every file and folder** in src/storage/mod.rs remove_entry. Manager awaits the
whole package removal, while retaining its shared installation lock. UI awaits
the final response. A focused candidate fix is to flush the containing directory
after the selected subtree has been removed, retaining safe descriptor-based
traversal and the separate durable credential/config removal. The user subsequently approved and the fix was implemented; see optimized evidence.

Rust's [official implementation](https://doc.rust-lang.org/src/std/sys/fs/unix.rs.html)
uses F_FULLFSYNC for this operation on Apple platforms; plain fsync benchmarks
would not measure exactly the same primitive.

## Captured Everything events

```text
running 1 test
test real_install_echo_and_delete_latency_demo ... DELETE_DEMO {"details":{"installation_id":"installation-c1f3c54b09c3cc1f5c6f259288bc5c85"},"stage":"installation_accepted"}
DELETE_DEMO {"details":{"elapsed_ms":5593,"files":3870,"folders":611,"package_version":"2026.8.31","server_id":"server-99feec5b0b9a6eb5560b5922d66b1fbd","tools":13},"stage":"installed_and_echo_verified"}
DELETE_DEMO {"details":{"files":3870,"folders":611},"stage":"delete_started"}
DELETE_DEMO {"details":{"elapsed_ms":1021,"package_exists":true,"remaining_files":3666,"remaining_folders":600},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":2040,"package_exists":true,"remaining_files":3435,"remaining_folders":597},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":3059,"package_exists":true,"remaining_files":3222,"remaining_folders":587},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":4078,"package_exists":true,"remaining_files":3011,"remaining_folders":578},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":5096,"package_exists":true,"remaining_files":2825,"remaining_folders":543},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":6114,"package_exists":true,"remaining_files":2639,"remaining_folders":503},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":7130,"package_exists":true,"remaining_files":2441,"remaining_folders":477},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":8147,"package_exists":true,"remaining_files":2269,"remaining_folders":421},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":9161,"package_exists":true,"remaining_files":2117,"remaining_folders":353},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":10173,"package_exists":true,"remaining_files":1953,"remaining_folders":305},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":11184,"package_exists":true,"remaining_files":1792,"remaining_folders":241},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":12194,"package_exists":true,"remaining_files":1630,"remaining_folders":190},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":13204,"package_exists":true,"remaining_files":1424,"remaining_folders":174},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":14212,"package_exists":true,"remaining_files":1205,"remaining_folders":163},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":15219,"package_exists":true,"remaining_files":989,"remaining_folders":154},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":16226,"package_exists":true,"remaining_files":792,"remaining_folders":121},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":17232,"package_exists":true,"remaining_files":607,"remaining_folders":77},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":18236,"package_exists":true,"remaining_files":399,"remaining_folders":66},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":19240,"package_exists":true,"remaining_files":207,"remaining_folders":46},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":20242,"package_exists":true,"remaining_files":22,"remaining_folders":7},"stage":"deleting"}
DELETE_DEMO {"details":{"elapsed_ms":20383,"keychain_used":false},"stage":"package_cleanup_finished_before_fixture_key_delete"}
DELETE_DEMO {"details":{"elapsed_ms":20400,"response":{"next_steps":[],"server_id":"server-99feec5b0b9a6eb5560b5922d66b1fbd","status":"deleted"}},"stage":"delete_completed"}
DELETE_DEMO {"details":{"keychain_used":false,"package_removed":true,"runner_removed":true,"runtime_preserved":true,"saved_record_removed":true,"source_unchanged":true,"unrelated_file_unchanged":true},"stage":"preservation_verified"}
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 26.02s
```

Independent Evaluator ACCEPTed both runs, confinement/preservation evidence and
scoped diagnosis. After evidence retention, both temporary test roots, copied
runtimes, packages, caches, failed-attempt state, outside disposable canary and
task pointer were removed. All four owned paths were verified absent, with no
owned process remaining. Existing everyday Fetch/Node/Railway processes remained
running. The baseline investigation is complete; the subsequently approved production fix
and its measured improvement are documented in optimized-evidence.md.
