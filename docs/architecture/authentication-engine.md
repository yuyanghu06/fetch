# Shared credential inputs and verification

## November 3 — Shared helper credential storage, approved implementation

User asks whether the missing Google handoff can be reusable for other callbacks.
The user replied go to the shared internal credential-storage bridge, with
Google as the first adapter. This supersedes the earlier Google implementation
deferral for this bounded work. Use fixtures/dedicated credentials and preserve
isolation; no personal credential use or everyday-client changes are authorized.

The callback normally supplies a temporary authorization code; the login owner
exchanges it for credentials. Preserve that owner's callback/validation and keep
Fetch's existing remote OAuth implementation. The reusable bridge covers helper
credential reads/writes and renewal, not replacement callbacks or another OAuth
engine. Existing backend-owned OAuth can keep calling CredentialStore directly.

Approved bounded mechanism:

- Reuse the current per-server encrypted CredentialStore and assigned manual
  credential inputs. Do not import personal configs/token files or use helper
  fallback credentials. No new public secret-writing route or UI is proposed.
- Launch supported helper/server processes with a private channel bound to the
  owning server ID and operation/attempt. Adapters map their supported storage
  interface or narrowly scoped documented token-file access into shared reads/
  updates. They cannot select another server's account/store or an arbitrary
  filesystem destination.
- Acknowledge writes only after encrypted persistence/readback. Initial login
  and normal MCP-process renewal use the same owner/store; serialize updates so
  an older snapshot cannot overwrite newer token changes. Support actual helper
  token formats, including Google's refresh-token-only payload, without assuming
  an access token is written by every helper.
- Preserve pending ownership, callback validation, original expiry/cancellation
  and safe error reporting. Cancelled/expired/stale attempts cannot claim success
  or overwrite a newer attempt. Keep per-server work asynchronous so other
  servers/UI remain responsive. No plaintext fallback or silent method retry.
- Before authenticated success, verify access using the stored credential under
  the existing D25/D31 requirements. A browser success page, process exit or
  storage acknowledgement alone is not proof. Renewal uses the same required
  verification/ownership rules as existing authentication.

Each adapter still supplies the helper command, storage-hook/path/format mapping
and required service check. Arbitrary binaries do not automatically offer these
interfaces. Installed Google1.11.3 has no storage hook and needs the specifically
reviewed adapter for its existing callback/helper and normal MCP child. Do not
claim universal compatibility or implement broad file interception by assumption.
Hosted MCP/server-owned third-party credentials remain with their owner; do not
assume every callback exports provider tokens to Fetch.

This adds a shared helper-storage boundary. Existing Authenticate remains the
public operation; no server-specific route/login engine, manual secret channel
or plaintext-token mode. Shared source is implemented:176 unique checks and native compilation pass;
independent Tester/Evaluator ACCEPT. Real Google consent/account/native
acceptance remains unverified.


## November 3 — shared startup/authentication dispatch

The user selects routing from saved imported command, arguments and environment
requirements, not README interpretation. The existing Authenticate HTTP/MCP
operation calls common setup functions: remote HTTP defaults to web OAuth unless
trusted compatibility data requires manual credentials; API inputs use assigned
encrypted entries and native Settings; supported CLI/local helpers reuse their
existing login. Do not guess arbitrary CLI commands or execute free-form text.
A failed selected flow returns its safe error through existing responses/status;
it does not automatically fall back to a different authentication method.

Every instance owns its configuration, authentication method, encrypted
credentials, pending attempts and renewal separately, including multiple
instances of the same MCP package using different accounts. Package identity
selects compatibility code, not an account or shared credential store. The
existing source-path/name installation reuse remains unchanged in this refactor;
explicit duplication behavior needs its own complete contract if introduced.

Google is now the first approved local-helper integration. Current installed
1.11.3 was reverified in package-45d4c35aec3aba7fe57de2e84813db00; its auth.js
hash matches the earlier inspected distribution. It writes refresh-token-only
state and has no storage hook. The shared private bridge and Google adapter are
implemented and fixture-verified; no live account authentication is yet verified.
Browser success alone cannot establish authentication. App credentials may come
only from assigned encrypted entries; no personal credential or token migration.

## November 3 — connection result versus a required login

D83 selects a default connection attempt at the end of successful installation,
through the same backend operation as Restart / Connect. Successful servers stay
running. GET /api/v1/servers remains the read for runtime and authentication.
When a fresh credential-free connection succeeds without a known login requirement,
authentication.status is not_required and the UI hides its auth controls. Unknown
alone is not evidence that the user must log in. Required missing/rejected access
and genuine runtime failures remain distinct; no browser is opened automatically.

not_required describes that usable connection; it is not authenticated provider
proof or a promise that every protected tool works anonymously. Existing D25/D31
checks still apply to stored credentials. The new source must preserve later
explicit auth challenges, pending credential changes, successful installation
records, ready reuse and shutdown. No new HTTP routes or secret boundaries.

D82, October 30, 2026. This describes the internal refactor. It adds no public
route, caller-supplied verifier URL or saved-record migration. D25/D31 still
require the current stored credentials to pass their authenticated check before
Fetch reports authenticated or a successful credential save.

## Read names from the imported configuration

`src/manager/bindings.rs` collects all supplied environment/header names and their
server-local credential references. It does not recognize provider names.
Environment names are case-sensitive; header names are case-insensitive. Duplicate
source names are rejected rather than choosing an arbitrary value.

The encrypted store remains the same per-server store. The shared input resolver
reads only assigned values and supplies them to that server's environment or
HTTP headers. Generic input storage/delivery does not establish authentication.
An unknown connector can receive its assigned inputs while Fetch reports that
its authentication proof is unavailable.

## Describe what the authenticated check requires

`src/manager/verification_profiles.json` is trusted repository-owned data compiled
into Fetch. Existing profiles describe Tavily, GitHub, Linear and Railway. A
profile chooses the applicable package or HTTPS host, the relevant imported
input names, the HTTP method/destination, input placement, and success condition.
These selectors specify proof inputs, not which arbitrary values may be stored.
Login mode is independent: a new HTTP description defaults to standard remote
OAuth, while existing manual-only/Linear compatibility modes are explicit trusted
data. Adding proof must not disable generic browser sign-in.

A check may test each selected key separately or combine several values in one
request. Header and JSON-body placements support a token plus an account/project
value. Static headers/body and explicit response predicates describe service
requirements without a connector-specific HTTP function. A future supported
checker can be added as reviewed data when this request model fits; a wrapper's
private browser-login/refresh interface may still require separate integration.

`src/manager/verification.rs` binds that description to the actual record and one
encrypted readback, then builds requests and checks responses. It requires 2xx
and all declared body predicates. Linear retains a string viewer id and absent
errors member; Railway retains a trimmed nonblank subject. GitHub and Tavily
retain their current authenticated 2xx policy.

## Keep each proof tied to the saved inputs

Every authentication value submitted for a checked save must participate in the
bound plan. Missing/ambiguous input bindings, unused inputs and overlapping
request destinations fail. An older OAuth token cannot prove a new manual key;
the new key's check and applicable effective OAuth check must pass. All required
values and complete HTTP requests are validated before any check is dispatched.
The existing per-server lock and request ownership remain in place.

Ordinary client configs, environment values, MCP metadata/tool text and API/model
arguments cannot add or replace the verification destination. Unit tests have
compile-time test-only profile/endpoint hooks; production has no such setting.

## Separate generic OAuth from service proof

The existing remote OAuth module continues metadata discovery and token ownership/
renewal through the same encrypted store. Railway's documented native callback
and private CLI setup, and the existing Linear login compatibility path, remain.
This refactor does not make private local-wrapper logins standard, add Google or
Slack proof automatically, or make ping/tools-list readiness count as D25 proof.

No description means no verified authentication result. Removal-only, unbound and
ordinary non-auth saves retain their unresolved D31 outcomes. These operations
can commit storage and still return verification failure.

## Verification status

Independent 17 new cases,35 targeted manager cases and110 combined checks pass;
the locked native binary builds in 0.43s. Three regressions were added after
review found profile/login-mode coupling and unrelated-header auth evidence.
Independent source/docs evaluation ACCEPTs this scoped change. Delivery is
recorded in the generic-authentication milestone. See [durable evidence](../../milestones/generic-authentication/evidence.md).
Mocks/disposable keys only; no real provider/native/MCP/client use is established.


Shared helper implementation evidence: the canonical encrypted helper section
contains adapter/revision/payload/verified_revision, without an OAuth mirror.
Per-process leases own socket/capability lifetime; private failure notifications
recheck exact session activity after acquiring live state. Rechecks invalidate
old persisted proof first. Auth-relevant changes retire owners and invalidate
proof; execution-only changes keep renewal usable. New owners/checked app-input
changes force the library to refresh using current encrypted inputs. Existing
Authorize/Settings contracts and raw MCP output remain.
[Evidence](../../milestones/shared-mcp-authentication/helper-storage-evidence.md).
