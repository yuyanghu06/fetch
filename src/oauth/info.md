## November 12 — exact project resources

Browser codes, native requests, families and refreshes retain one exact registered
global/project MCP resource. Caller visibility intersects current saved server
membership. History permission is project-only and unchecked until selected in
both approval screens. Safe native responses show actual project name/folder;
claims include binding for profile validation. Fake-key fixtures prove project
code/refresh retarget denial, reopen persistence and revocation; no actual
Keychain/browser confirmation was exercised.

## November 12 — native connector approval

connections.rs adds the four connection-request APIs used by the local stdio
connector and Fetch's approval popup. Creating/polling a request grants nothing.
Only a live native Settings session can read the safe pending list or decide;
Allow confirms fresh Mac presence before marking the selected scope subset
claimable. A separate private claim proof retrieves credentials once. Lists,
request codes and guidance never contain that proof or issued credentials.

Browser PKCE and native claims use the same grant-family constructor and encrypted
application persistence. Native connector clients have refresh permission without
invented redirect URIs; existing OAuth refresh/revoke and ownership checks apply.
Pending requests are memory-only and disappear on Fetch shutdown/restart; approved
token families remain durable. The connector does not receive native Settings
authority. has_pending_connections is a nonsecret window wake-up signal only.

connections_tests.rs contains three disposable route flows for native permission
selection/claim ownership, distinct clients with refresh/revoke/restart, and
denial/expiry. Fake keys and native confirmation avoid Keychain/OS dialogs. The
first compile found a test router-name collision, which is corrected; coordinated
testing is owned by the Orchestrator and no passing runtime result is claimed here.

## November 6 — Chrome callback policy

The approval page permits form submission to Fetch itself plus only the validated loopback callback origin for its pending request. Chrome applies form-action to the303 callback redirect; self-only blocked the real client after native confirmation. Callback origin comes from the already-validated registered request, with exact scheme/host/port and no wildcard. Existing exact redirect/path/token checks remain; other OAuth pages retain the fixed restrictive policy.

## November 6 — approval form origin

Approval HTML uses same-origin referrer policy so normal browser form submission retains the exact Origin required by the existing checks. Redirects and token/error responses keep no-referrer. tests.rs exercises registration, rendered form fields/cookie, origin refusal, fixture native approval, callback and PKCE code exchange through the full API router using disposable state and fake keys. It does not use real Mac approval or client credentials.

# Connecting apps to Fetch

D81 browser rendering now uses the shared escaped document/form/permission
helpers in src/browser.rs. Labels/style/default scope choices and OAuth security
headers are unchanged; management remains explicitly unchecked. Build/fixture
acceptance does not establish real Mac approval or browser/client trust/login.

This folder implements account-free app authorization. Clients register loopback callbacks, open a browser approval page, and exchange a Mac-confirmed PKCE code for scoped access and renewal tokens. Management permission is separately requested and unchecked on the approval page. No Fetch password or account is created.

Registrations and token-family hashes are encrypted in the application store. Refresh rotates once; reuse of a spent refresh token durably revokes that family. Standard revocation affects the entire family and emits its owner ID so the runner can stop delivery and cancel its owned work. Pending browser requests and codes last only for this process. OAuth responses retain standard formats without application `next_steps` wrappers.

The parent router supplies protected-resource metadata and the shared Host/Origin boundary. This module supplies authorization-server metadata, registration, authorization GET/POST, token and revocation routes. Native Settings session headers are recognized only if the in-process window minted a live session; they cannot be obtained through OAuth.

All new code in this pass is untested at the user's request. No app, provider, Keychain, browser, service, HTTP call or test was started.

## Automatic first connector approval — November 12

Creating the existing native connection request now emits a one-shot in-process wake signal so the UI foregrounds its approval popup. It issues no grant.
