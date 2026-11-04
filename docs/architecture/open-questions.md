# Current decisions and remaining work

November3,2026. User's “you're good to start” resumes the scoped refactor/testing.
The [current contract](native-server-storage.md) supersedes the old encrypted-provider
requirements. [Older checkpoint text](../../milestones/native-server-storage/historical-open-questions.md)
is historical and does not pause this work or reopen settled choices.

## Settled

- Native server/helper owns its login, callbacks, files and refresh. No patches,
  Google token interception, private bridge or per-server refresh implementation.
- Official Rust MCP library owns standard online-client login and refresh; direct
  Keychain saving is approved for that unavoidable Fetch-held access.
- User and background requests use the actual existing authenticate route. Quiet
  requests cannot open the browser or launch native setup.
- Retain native Settings-only credential PATCH for manually entered fixed keys;
  successful storage returns stored and verified:false. Values are never returned.
- Config PATCH/MCP config tool edit Fetch execution settings only. GET servers
  reports nullable access expiry/check times without reading tokens or doing login.
- Open server folder uses the recorded actual native state location; no package/HOME
  guess. Recorded native setup commands run exactly; no README interpreter.
- Preserve old files/keys/packages/credentials, personal clients, HTTPS, callback
  checks, Fetch's own /oauth/token, permissions and unchanged raw MCP payloads.

## Remaining decisions and acceptance

1. **Native command/path authoring:** the internal recorded-command runner exists,
   but ordinary imports have no approved way to supply exact setup commands and
   state folders. Present any new import/UI/API fields or concrete per-server
   handling before implementing them. Unknown records return clear unsupported
   results. Railway's existing extra-CLI preparation is retained; new native setup
   support needs concrete review.
2. Finish independent review fixes/retests/integration for the scoped refactor.
   Initial fixture/native build and confined unmodified Everything check pass;
   three authentication recovery/reconnect issues are being fixed.
3. Actual native Keychain, Finder, Mac confirmation, provider-account login/reuse,
   coding-client and visual acceptance remain unrun. Use the protected environment
   and dedicated accounts; do not treat passing fixtures as that acceptance.

Shared memory/history, PostgreSQL/retention, graph, extra server processes and release
work remain deferred. No new watcher, automatic file-change restart, credential
migration/decryption/deletion, personal config edit or release is authorized.
