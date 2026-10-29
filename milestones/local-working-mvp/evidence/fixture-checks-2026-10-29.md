# Independent fixture checks — October 29, 2026

Acting role: independent Tester. D81 authorizes these checks; the Orchestrator
explicitly gave the execution go-ahead after the integrated native binary build.
Only assigned test/evidence files changed. No application source was fixed by
the Tester. These results concern the current shared working tree, not a clean
released commit.

## Commands and observed outcomes

First command:

```sh
cargo test --locked --lib --test backend --test raw_json --test environment_paths --test import_compatibility --test local_mvp_acceptance
```

Compilation completed in 7.76 seconds. Exit status was 101 because one import
test failed; Cargo therefore did not reach the later local-MVP/raw targets.

| Target | Observed result |
| --- | --- |
| Library | 21 passed, including six remote-auth security cases and two shared browser-rendering cases. |
| Backend API | 16 passed. |
| Development/release paths | 4 passed. |
| Import compatibility | 6 passed; 1 failed. |

Because the failure stopped the combined command, the remaining targets ran
separately:

```sh
cargo test --locked --test local_mvp_acceptance --test raw_json
```

Exit 0: all 11 independent local-MVP cases and the existing raw-JSON case passed.

```sh
cargo test --locked --test development_https
```

Exit 0: all seven HTTPS cases passed. Compilation completed in 3.36 seconds;
the cases finished in 0.05 seconds. Distinct total across the commands is 67
cases: 66 passed and one failed. No case was ignored.

## Failure and required correction

`documented_public_endpoints_capture_without_exposing_or_rewriting_the_url`
failed at `tests/import_compatibility.rs:37`:

```text
assertion failed: !scan.to_string().contains("/mcp")
```

The broad substring also occurs in an allowed origin such as
`https://mcp.miro.com`. This assertion cannot distinguish a safe hostname from
an exposed endpoint path. The test owner must assert the projected fields and
allowed origin precisely while still checking that source secret values and
private URL path/query data are absent. This failure alone does not establish an
implementation leak. The Tester reported it to the Orchestrator and did not edit
the Coder-owned import test or application implementation.

## Targeted retest after the test-author correction

The import test author replaced the broad substring assertion with an exact
projected connection check and an absence check for the full supplied endpoint.
No application implementation changed for this correction. The independent
Tester reran:

```sh
cargo test --locked --test import_compatibility
```

Exit 0: 7 passed, 0 failed, 0 ignored. Compilation completed in 2.60 seconds and
the cases finished in 0.01 seconds. The previously failing case passed. Latest
results across the selected targets are therefore **67 passed, 0 failed**.
This is the union of the recorded target runs, not a claim that the original
combined command exited successfully. No broader rerun was needed for an
assertion-only correction.

## What the independent new cases establish

### Subsequent combined run after the issuer-response fix

At approximately 12:10 America/New_York on October 29, the independent Tester
reran the full selected command against the frozen integrated source containing
the issuer-response fix:

```sh
cargo test --locked --lib --test backend --test raw_json --test environment_paths --test import_compatibility --test local_mvp_acceptance --test development_https
```

**Exit 0: 68 passed, 0 failed, 0 ignored.** Compilation completed in 15.96 seconds.
Target counts: library 22; backend 16; HTTPS 7; paths 4; imports 7; independent
local-MVP 11; raw JSON 1. The added library case
`authorization_response_requires_exact_published_issuer` passed. All previously
selected cases passed in this one combined run; the earlier failing command and
targeted retest remain preserved above.

The issuer helper case checks an exact match, missing issuer, different origin,
trailing-slash/case variations and invalid saved issuer-support metadata. Source
inspection confirms `finish_provider_callback` calls that helper before building
the token-exchange request. **This is not a complete callback-to-token-exchange
integration test.** A real pending `ProviderFlow` and live auth-attempt state are
private and have no public fixture injection seam; constructing them through
normal public operations would require actual metadata/registration/login calls.
No such calls or fake production approval were performed, and no application test
hook was added merely to claim this evidence. Actual provider acceptance remains
pending in the protected environment.

### Independent fixture behavior

- Catalog permission exposes only the catalog management tool and preserves
  the native MCP envelope.
- Catalog/tool-call permission cannot dispatch management operations.
- Missing authorization is denied before malformed protocol data is parsed.
- Modern method/name headers must match the protocol body.
- Duplicate JSON keys inside arguments are rejected before backend dispatch.
- Legacy methods require completed initialization; owner revocation removes
  the session.
- An ordinary management token cannot write manual secrets through the
  production native-authority check; denial creates no saved state.
- Partial modern catalogs retain valid raw definitions and legacy visibility;
  duplicate tool names remain rejected.
- Server-qualified aliases survive a disposable runner restart without
  confusing origins or the reserved Fetch namespace.
- Resource/template selectors preserve their original byte content and origin.
- Reads or calls against a configured stopped server do not start its executable
  and report `SERVER_NOT_READY` with `execution: not_started`.

## Protection and evidence limits

All records, keys and sources were disposable fixtures. MCP/API tests were
in-process; the configured nonexistent executable was never launched. HTTPS
cases used disposable loopback listeners on available ports and certificate
trust scoped to their own HTTP clients. There were no real MCP servers,
provider calls, external network requests, package installations, personal
client/config/credential reads or edits, production Keychain access, native
approval prompts, app launches, system trust changes or packaging/signing actions.

The build reported the existing four classic-Keychain deprecation warnings and
one unread protocol-error field. Passing these checks does not prove the native
window, real provider authentication, separate Codex connection or installed-MCP
workflow. Those require their own observed evidence in the protected environment.
