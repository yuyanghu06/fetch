# Verifying saved provider credentials

October28, 2026. **Historical unaccepted proposal; withdrawn from active review by D73.** D25/D31 already require successful encrypted storage, private readback and a service-authenticated successful request before credential-based authentication success. D72 settles the shown Settings/Keychain direction. The draft addresses verifier binding, timeout, failure/save outcome, batch/no-op/removal and non-auth policies, but none of its new clauses is adopted by the plan consolidation. Do not request another broad login approval from this document; carry concrete engineering requirements in the single implementation worklist. It uses the existing authenticate POST, credentials PATCH and server GET; no test/status/tool route is added.

## The success gate stays

For an authentication credential, save the complete encrypted result, read back its assigned values privately, use those values in the reviewed harmless authenticated request and require service2xx **plus its documented success meaning**. Every authentication entry submitted in `set`, including an unchanged value, must actually participate in a covered credential group. Test all required resulting authentication groups in this verification pass; prior successful evidence is not a shortcut. A pair of values such as username/password may share one joint test; unrelated credential groups need their own proof.

No public health/ping, generic `tools/list`, arbitrary operational tool or still-running process retaining an old environment can replace the gate. Readback/config/credential-group generations must still match when publishing the result. Late or superseded evidence cannot authenticate newer saved values. Do not change raw Tool definitions/arguments/results for this Fetch-owned control operation.

## Reviewed verifier binding and preflight

A Fetch-owned supported integration record must identify the credential owner/token class and entries, exact endpoint/method/headers/body, expected account/workspace context, minimal required authority, success predicate and private delivery interface. It must also identify refresh ownership and restart behavior. These are supported adapter facts, not guessed provider text, a model argument or a caller-supplied test URL. No arbitrary redirects or cross-origin credential forwarding.

Before a manual write, validate the entire batch and whether every authentication SET can be verified. Unknown/unbound/unclassified SET values are **not automatically non-auth**. Reject unsupported credential SET operations before writing with422 `CREDENTIAL_VERIFICATION_FAILED`, safe setup guidance and unchanged prior values/evidence. This is a new visible restriction on storing arbitrary unbound values. Configuration may first save a missing credential reference under D30, but a compatible verifier still must exist. Do not require a catalogue or normalization of arbitrary native server settings.

Known server + missing verifier is not404;404 still means unknown server. Authenticate retains its accepted200 `required`/`unknown` guidance branch if no controllable automatic integration exists, rather than pretending a failed login started. A supported flow needing a test must know its verifier before claiming support.

Wrapper-owned opaque OAuth needs a reviewed private storage/provisioning interface meeting D25/D26. Do not scrape caches/stdout/tools or assume generic provider-token export. Installation remains distinct from claiming supported automatic login/persistence/renewal.

## Thirty-second check and failure mapping — proposed

Propose **30 seconds total**, starting immediately before the first post-save readback and covering readback, private delivery and all required service probes. Human confirmation and initial save are outside this provider-test budget; their own reviewed deadlines apply. An accepted D27 authentication flow uses the smaller of30 seconds and its remaining overall deadline. Never reset the timer per group/network stage.

One verification attempt; no automatic write replay, new browser login, provider probe retry or ordinary-server restart. Any provider helper/refresh sequence must be part of its separately reviewed agreement, not an implicit retry. Status GET performs no fresh readback/probe and does not reset a deadline.

| Outcome | Proposed HTTP / existing error family |
| --- | --- |
| Unsupported/unbound SET before save, missing/rejected required credential, or proof of current stored values cannot be established | `422 CREDENTIAL_VERIFICATION_FAILED`. Before-save rejection preserves prior values/evidence. |
| Service unavailable, transport/TLS failure, redirect or unusable/invalid service response | `502 CREDENTIAL_VERIFICATION_FAILED`. |
| Verification deadline exhausted after successful stored readback | `504 CREDENTIAL_VERIFICATION_FAILED`. A readback-stage failure/timeout remains the storage error below. |
| Credentials PATCH encrypted save/readback fails | Existing `500 CREDENTIALS_WRITE_FAILED`; no provider probe or fake verification result. |
| Accepted asynchronous auth later fails its required test | Existing server GET remains200, with `authentication.status: failed` and shared `authentication.error.code: CREDENTIAL_VERIFICATION_FAILED`. |

Provider401/403 or a documented service-level rejection means422 here; it must not challenge the app to reconnect to Fetch. Fetch caller401/403 remain its own authentication/permission failures. A service-level error inside a2xx response still fails the gate. Safe messages explain the cause without credentials/provider response dumps.

Other D27 auth startup/storage/denial/cancellation/overall-expiry failures retain their established codes and accepted async behavior. This amendment does not turn an encrypted-store failure into provider rejection. MCP management tools retain D49's safe application JSON/text/error mapping; a GET successfully reading a failed auth state is still a successful read.

## Save outcome, retention and current authentication state

Save the whole batch atomically, not one successful entry at a time. If verification then fails, **retain the newly saved encrypted values** and report failure. No automatic old-value restoration, provider revocation/deletion or restart. A later explicit authenticate operation can test the stored values without blindly repeating a write. Other servers and unrelated credentials are unaffected.

Propose these fields on credentials PATCH success/failure, retaining its existing result/error/affected-name/restart/next_steps shape:

- `credentials_stored: true|false|null`: true means the requested resulting state was confirmed in durable storage, including an unchanged state/removal; false means this request is known not to have committed or established that state; null means its persistence outcome is uncertain. False does not mean no older credential exists.
- `changed: true|false|null`: whether this request changed the stored entries; null only where the commit outcome cannot be established. Existing successful results keep the current boolean meaning.
- `verification: verified|failed|not_applicable`: verified means all required checks passed; failed means they did not or could not; not_applicable is restricted to the explicit exception below. Storage failure must not invent a completed provider-test result; omit verification when the gate was never reached, except a known preflight inability may report failed safely.

| Case | Stored / changed / verification |
| --- | --- |
| Preflight rejection | false / false / failed; preserve previous saved values and evidence. |
| Changed authentication batch saved and all checks pass | true / true / verified; authenticate the exact current state. |
| Unchanged authentication value submitted and check passes/fails | true / false / verified or failed; no prior-success shortcut. |
| Changed batch committed then provider check fails | true / true / failed; retain values and mark current auth failed/unverified with the shared error. |
| Save outcome cannot be confirmed | null / null / verification omitted; no success or blind automatic retry. |

Any actual manual store change retains the existing base behavior of invalidating older authentication evidence. The check may establish new current evidence; the exception below cannot. Required-value deletion yields required/missing guidance; unknown integrations remain unknown and known no-auth paths remain truthfully not_required. Running processes may still hold older values under the existing saved-versus-applied/restart contract.

## Explicit narrow D31 amendment — proposed, not yet accepted

Deleting a value or saving a known non-auth setting cannot prove a new login credential works. Propose200 for **storage completion only**, with `verification: not_applicable`, for:

1. A pure removal batch, including removing an already-absent name.
2. SET changes/no-ops consisting only of explicitly reviewed non-auth values.
3. An empty object/empty set-remove arrays: valid no-op under the base contract, no provider test and no authentication promotion.

Return the existing safe names/status/restart fields, confirmed stored outcome and changed flag; no secret values. These branches never establish authenticated. Actual changes invalidate prior evidence as above; a genuine no-op preserves existing state but does not refresh its proof. They do not execute a public health check or a provider tool to manufacture success.

Any batch containing an authentication SET, even identical to the saved value, still needs the full gate; mixed auth/non-auth/removal batches do not get the exception. An unknown/unclassified/unbound SET is rejected before save, not silently exempted. Known no-auth/native-permission integrations use their own reviewed no-auth/permission status/check without inventing HTTP credential proof.

This is an explicit proposed amendment to D31's broad test-before200 wording. If not selected, the strict alternative is a successfully saved deletion/non-auth change returning a non200 verification failure. Do not evade that choice with204/202 or claim the exception already exists.

## Concrete service examples and remaining integration scope

- GitHub **user/PAT token** example: `GET https://api.github.com/user`, bearer from stored readback; require200 and the documented authenticated-user response. This proves that user credential works, not repository permissions or installation-token validity. [GitHub endpoint](https://docs.github.com/en/rest/users/users#get-the-authenticated-user).
- Slack example: `POST https://slack.com/api/auth.test`, bearer from stored readback; require2xx **and `ok: true`**. `ok: false`/`invalid_auth` fails even under HTTP200. [Method](https://docs.slack.dev/reference/methods/auth.test/), [response semantics](https://docs.slack.dev/apis/web-api/#evaluating-responses).

These source-backed examples illustrate proposed verifier semantics, not acceptance of complete GitHub/Slack automatic integrations or the GitHub CLI tool bridge. Exact source-specific ownership/private delivery/refresh and supported bindings still need their own complete review. The [GitHub CLI proposal](github-cli-proposal.md) retains its separate scope/auth ownership gaps; memory-only wrapper OAuth does not meet encrypted persistence merely because a browser completed. No new provider credentials may be migrated from personal clients.

UI sequencing and next_steps must agree: unsupported preflight changes nothing and shows missing support; a committed/tested save can show verified; a committed failed test must say saved-but-not-verified; pure removal/non-auth/empty success says storage-only and the truthful auth state. GET reads the latest known evidence without new network work. No provider result/secret or rejected value enters messages, logs or tool outputs.

Future authorized D46-isolated evidence must cover unknown SET rejection/no write, exact new stored values/private handoff, multi-group total budget,2xx semantic rejection, no-op/removal/empty/mixed branches, committed-failure retention/uncertain outcomes, stale evidence and auth/MCP/UI failure mapping. No live probe, credential access, server/client launch, Keychain/certificate/environment change or dependency installation occurred while drafting.
