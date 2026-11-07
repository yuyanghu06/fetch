November6 discussion: user proposes exploring server-first tool discovery after actual Codex reports Fetch connected with405tools. Context cost is unmeasured; catalog count alone does not prove full definitions are always in model context. No catalog/API/visibility/connection change selected. See [research proposal](research.md#november-6--405-tool-catalog-and-per-server-discovery-proposal).

November4: publicclientidentity andHTTP127callback selected/implemented/deployed/independentlyaccepted. FixedpublicURL https://www.yuyang-hu.org/oauth-client.json; nohostingchoice remainsopen. ActualVercelaccountsign-in isstillruntimeacceptance, not provenbyfixtures. See milestones/client-identity/evidence.md.

November 4: custom issuer-feature and other redundant pre-login gates are selected for removal under the wrapper design. Library validation and existing application boundaries remain. See milestones/wrapper-authentication/info.md; no new API or provider-specific flow selected.

# Current decisions and remaining work

November 4: user explicitly authorizes service-startup and rejected-response console logging, superseding the earlier explanation-only pause for this scoped feature. Existing native-only memory behavior remains, with fuller attempt output and explicit truncation; no new auth method/provider tests. See native-server-storage.md and milestones/service-startup-logs/.

November 4: user approved installer-recorded private executable folders in the shared startup environment, including existing installations through an in-memory known-layout fallback. This is program discovery, not a new login method. [Current scope](native-server-storage.md#november-4-selected-private-program-search-folders).

November4 startup decision supersedes earlier required-input guidance: Fetch
starts/connects with present saved environment/header values and omits missing
ones for all startup paths. No mandatory-input inference. [Current amendment](native-server-storage.md#november4-selected-startup-behavior).

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

## November3 selected startup/login work

Later user steering selects existing /restart start/connect route for the button,
with interactive:true; /authenticate stays as deprecated compatibility. No new
/start route. See current wrapper amendment.


User says “lets try that then”: implement recorded helper or ordinary server
startup, visible safe login interaction, last-known/current distinction and
truthful unsupported guidance. [Exact selected follow-up](native-server-storage.md#november3-selected-authenticate-follow-up--implementation-underway).
Source/test work is complete for the scoped startup/link change; prior research-only
waits are superseded.171 independent checks pass and Evaluator ACCEPTs this scope. Native URL elicitation reply/ownership remains unselected; do not
claim universal arbitrary-server authentication.

## Remaining decisions and acceptance

November3 research follow-up: [existing clients and login libraries](mcp-login-discovery-research.md)
find no reviewed universal native-helper discovery mechanism. Google already
starts login on normal startup when needed; Fetch discards its printed link.
Review how users receive server-owned setup interactions as well as any separately
required command records. A command record alone is not a complete Google fix.
These are findings/recommendations, not approved source or API changes.

1. **Native command/path authoring:** the internal recorded-command runner exists,
   but ordinary imports have no approved way to supply exact setup commands and
   state folders. Present any new import/UI/API fields or concrete per-server
   handling before implementing them. Unknown records return clear unsupported
   results. Railway's existing extra-CLI preparation is retained; new native setup
   support needs concrete review.
2. Scoped refactor accepted and integrated:151 checks, separate confined original
   Everything check and native binary build pass. Independent review fixes resolved.
3. Actual native Keychain, Finder, Mac confirmation, provider-account login/reuse,
   coding-client and visual acceptance remain unrun. Use the protected environment
   and dedicated accounts; do not treat passing fixtures as that acceptance.

Shared memory/history, PostgreSQL/retention, graph, extra server processes and release
work remain deferred. No new watcher, automatic file-change restart, credential
migration/decryption/deletion, personal config edit or release is authorized.

## Separate development identity work

The concurrent development setup task verified the selected stable signer and
process cache for existing application keys. Source/fixture/signature evidence
passes; native prompt-count/lock/denial proof remains. A shared wrapping key,
credential migration and release work remain unselected. See
[keychain prompts](keychain-prompts.md). This refactor does not repeat those
separate native signature checks or claim the new Keychain flow was exercised.
