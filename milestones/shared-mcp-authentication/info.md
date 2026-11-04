# Shared asynchronous MCP authentication

Current November3 state: shared credential bridge and Google1.11.3 adapter are
implemented.176 unique fixture checks, native build and independent Tester/
Evaluator ACCEPT. [Storage evidence](helper-storage-evidence.md) includes initial
fixture failures/corrections, cleanup and limits. Real Google consent/account/
native acceptance remains untested. This supersedes earlier Google deferral
notes below; broader authentication work remains unfinished. Delivered asd19d891
after integrating newer discovery. Final188 checks/native build/independent
review pass; owned test state cleaned.

User request November 3: main agent must implement this after delivering the
delete-server route. Side conversation researched/recorded direction only.
Status: deletion and × delivered. User completed scope sync: select shared startup/
authentication functions from imported command, arguments and environment needs;
remote defaults to HTTP/web OAuth. Return selected-flow errors without silent
alternate-method fallback. Common dispatcher/OAuth source is implemented;
independent Tester/Evaluator ACCEPT,99 unique fixture checks and locked native
build pass. [Evidence](evidence.md) retains the initial stale-alias fixture
failure/correction and real-provider/native/client limits. Source/evidence
delivered as4f18905; live remote/local commit matched.
Installed Google1.11.3 requires an encrypted storage bridge; the complete private
bridge is explicitly deferred by the user; finish routing first. No Google-specific route.

Read current AGENTS.md, NOTES.md's November3 "Reuse upstream MCP authentication
implementations" and docs/architecture/authentication-engine.md FIRST, then
inspect current source because it changes during discussion.

Goal: existing Authenticate API/tool becomes shared asynchronous orchestration
for standard HTTP MCP OAuth (discovery plus existing app credentials as needed),
URL-mode elicitation (server URL/consent/callback/third-party credentials stay
server-owned), and documented local-package login helpers. Google is first local
helper, not another large provider-specific Rust branch. Share browser opening,
pending ownership, cancellation/expiry/errors/completion; UI/other servers stay
responsive. Preserve raw MCP data, existing authorization/encrypted stores and
Settings-only manual writes. New public contracts/major behavior need full user
proposal; do not replace a helper-owned callback without helper support.

Upstream reuse requested: pin exact source revisions, inspect dependencies,
reuse suitable portable source and preserve licenses/notices/attribution.
- Codex Apache-2.0 Rust: https://github.com/openai/codex/blob/f1b0721c88002066201e7d977c449fbf14f1e499/codex-rs/rmcp-client/src/perform_oauth_login.rs
- OpenCode MIT v1.19.4 startAuth/URL/authenticate/finishAuth: https://github.com/anomalyco/opencode/blob/148842ef7cb1bc34192b3fff57d0f9721ab9c801/packages/opencode/src/mcp/index.ts#L806

Google: actual Fetch-installed @a-bonus/google-docs-mcp1.11.3 verified by
read-only package/source inspection. Its auth helper prints URL on diagnostics, owns temporary
localhost callback, exchanges code and saves tokens AFTER browser success page;
that page is not proof of completion. Inputs GOOGLE_CLIENT_ID/GOOGLE_CLIENT_SECRET/
GOOGLE_MCP_PROFILE. Existing personal definitions have app credentials; do not
assume a new Cloud app is needed and do not silently copy credentials/token files.
Use isolated Fetch-managed state per account; verify helper token format/renewal
and keep any encrypted import/handoff consistent with subsequent token changes.

Inspect Railway/Linear and reuse ordinary OAuth in the common implementation;
retain only real compatibility differences and preserve working behavior.

Verification follows current actual permissions and resource isolation. Temporary
MCP package/flow experiments may use disposable cache OUTSIDE repo/Fetch tests/
development/release state, with cleanup/process stop even on failure. Protect
personal configs/services and credentials. Verify remote/helper/account isolation,
cancel/failure/reuse/renewal where authorized. Distinguish source/mocks/actual
provider evidence; authentication success still requires applicable saved-access
checks. Deliver implementation/evidence/reuse attribution/concrete blockers;
maintain journals; no packaging/release.


November3 follow-up: Google stays pending by explicit user choice. Multiple
instances of the same MCP must support independent methods, credentials and
accounts (different Google accounts are the concrete example). Preserve
server-ID ownership; do not silently change same-source installation reuse.
User wants a behavior walkthrough after the routing implementation/checks.

## Current November3 authorization — shared helper bridge

User replied go to the proposed reusable bridge. The earlier Google deferral
is superseded for implementation of this bounded shared core and first Google
adapter. Existing helper/callback retained; same per-server encrypted canonical
helper payload and acknowledged renewal, with stored-access verification. No
public route/UI/engine/plaintext fallback or personal credential authority added.

Independent Researcher reverified installed1.11.3 from its current package and
actual nested OAuth2 dependency. Planner identifies separate lease gates to avoid
Manager startup-lock deadlock, per-process Runner cleanup, CAS/readback and
verification-before-runtime-renewal ACK. Core Coder owns credentials bridge/JS
client and minimal runner attachment; Adapter Coder owns Google/bootstrap and
Manager/verifier integration; independent Tester owns focused fixtures. No
actual provider/helper account login has run in this pass.
