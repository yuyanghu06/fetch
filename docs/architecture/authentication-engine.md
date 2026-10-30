# Shared credential inputs and verification

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
