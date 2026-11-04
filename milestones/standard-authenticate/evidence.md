# Standard Authenticate verification

November3,2026. Scoped implementation accepted; no universal server/provider or
real-account acceptance is claimed. No real Google account, native browser,
Keychain, everyday Fetch/client configuration or existing service was used.

## Implemented result

The existing /restart start/connect API accepts interactive:true for explicit
login. It uses a recorded helper if present, otherwise the configured local MCP
command. Normal startup and remote OAuth retain their earlier behavior. The native
Authenticate and Open sign-in at HOST controls call this start route. Legacy
/authenticate remains functional/deprecated and advertises /restart as successor.

Selected HTTPS sign-in links stay only in attempt memory. GET exposes host/opened,
not raw output, query strings or tokens. A second explicit consent action opens
the displayed host. Local interactive initialize can wait up to600seconds.
Ready/helper exit is not account proof. Terminal status/error is remembered using
the existing config field, with last_known separate from fresh current unknown.
Missing configured inputs identify names and direct the user to native Settings.
Atomic idle admission protects active/queued/uncertain work; Quit awaits local
attempts and their owned helper process groups. Pending UI is restored on reopen.

## Actual commands and results

| Check | Result | Evidence/limits |
| --- | --- | --- |
| cargo test --locked --lib |96/96 PASS | Fake browser/Keychain/native confirmation where applicable, scheduler idle/uncertain/queue tests, pending UI/reopen, remote OAuth/renewal fixtures. Corrected run0.37s tests. |
| cargo test --locked --test standard_authentication -- --test-threads=1 |8/8 PASS | Deny-default macOS confinement, existing outside/sibling/network canaries.33.85s tests including31.25s pre-initialize login wait. |
| cargo test --locked --test backend --test development_https --test direct_install --test environment_paths --test import_compatibility --test isolated_native_mcp --test local_mvp_acceptance --test raw_json |67/67 PASS | Counts24+7+10+4+7+2+12+1. Regression compilation2.91s; command wall7.34s. |
| cargo build --locked --bin fetch --message-format short |PASS | Initial15.41s; final frozen source0.30s incremental. Binary compiled, not launched or signed in this task. |
| Final guidance-only change: cargo test --locked --test backend --test standard_authentication --message-format short |32/32 PASS | Required-state agent guidance now recommends canonical start route. Backend24 in0.14s; confined8 in31.88s; compilation5.72s. No assertion weakening/retry/sleep used to hide completion race. |
| git diff --check |PASS | Final whitespace check. |

Unique independent checks:171 total (96 library+8 new confined+67 existing).
The final32 are a targeted recheck, not additional unique tests. Independent
Evaluator ACCEPTs final source and the route-guidance correction.

## New integration scenarios

- Actual router POST /restart interactive:true returns202 and status Location;
  GET stays responsive and reports host only. Deprecated /authenticate still
  works and includes dated Deprecation/successor Link.
- No native_setup is needed for ordinary configured startup. No duplicate
  process is created while waiting; immediate tool calls work after completion.
- A login before initialize survives the normal30second limit while another
  server remains usable; raw JSON large-number formatting stays intact.
- Explicit recorded helper URL output is handled without claiming account proof.
- Missing values report required both on ordinary Connect and explicit login.
- Current status remains unknown when loading a saved authenticated last-known
  result; no transient link, token or raw diagnostic output enters records/API.
- Corrected Settings inputs replace a registered stopped server's old environment.
- Active tool work prevents an account-changing helper; pending ordinary startup
  and helper Quit stop owned peers. Helper's forked descendant is reaped before
  shutdown returns.

## Findings retained rather than hidden

First library run:95/96 PASS; old test expected SERVER_AUTH_REQUIRED after the
intentional new SERVER_INPUT_REQUIRED behavior. Expected code/name guidance fixed.
First integration run:5/7 PASS. A real completion-order bug published terminal
auth status before call admission reopened, causing immediate SERVER_BUSY; fixed
and verified with unchanged immediate-call assertion. A nonexistent sibling test
canary returned ENOENT; fixture changed to an existing disposable sibling file.
Independent review also found pending-window consent restoration, actual atomic
idle-work protection, helper task ownership, input-status consistency and stale
registered inputs. All resolved and retested.

## Cleanup and remaining work

Tester verified0 task fixture/outside directories,0 matching confined processes
and0 test logs after its pass. Root's two final-check logs were deleted after
summarizing. No packages were installed and no provider credentials migrated.
Existing unrelated UI/research/docs edits were preserved; changes remain local.

Real Google consent, account/tool access and login reuse after Fetch restart are
still to be checked in the protected account environment. Native visual, browser
and real Keychain behavior were not observed. Native URL elicitation has no human
reply/ownership contract here and is not advertised. No helper commands are
inferred from arbitrary packages/instructions. Existing explicit metadata authoring
limits remain; a ready server with no sign-in request gives truthful guidance.

Use the signed development launcher after explicitly quitting an existing Fetch:
./scripts/run-development.sh. Closing the window only backgrounds the process.
