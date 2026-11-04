# Native server configuration and shared authentication

Goal: native servers own provider files/login/refresh; official rmcp owns standard
online-client login/renewal. Fetch keeps generic APIs, Keychain connection, process
control, authority, queues and unchanged raw MCP forwarding.

Status: scoped refactor complete and independently ACCEPTED November3,2026.
Final151 checks and locked native binary build pass. Separate original
Everything2026.8.31 confinement check passes1/1; all task-created packages/runtime/
caches/state cleaned. No full-MVP/universal provider acceptance claimed.

[Current contracts](../../docs/architecture/native-server-storage.md).
[Evidence, commands and limits](evidence.md). Historical planning and prior
architecture documents are preserved in this folder; they do not override current
contracts. Completion criteria for this scoped pass were implemented contracts,
state preservation, isolated verification, resolved material review findings and
independent acceptance. All are satisfied for this pass.

Remaining product work: ordinary imports cannot yet supply recorded native setup
commands/folders; real Keychain, Finder, Mac approval, provider accounts and coding
clients remain untested. Servers ignoring per-instance locations may share machine
credentials and are not promised independent accounts. Existing Railway CLI
preparation is retained; no new per-server auth code added. Preserve old keys/data,
packages, personal configs and services. No migration or release is authorized.
