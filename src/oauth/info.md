# Connecting apps to Fetch

D81 browser rendering now uses the shared escaped document/form/permission
helpers in src/browser.rs. Labels/style/default scope choices and OAuth security
headers are unchanged; management remains explicitly unchecked. Build/fixture
acceptance does not establish real Mac approval or browser/client trust/login.

This folder implements account-free app authorization. Clients register loopback callbacks, open a browser approval page, and exchange a Mac-confirmed PKCE code for scoped access and renewal tokens. Management permission is separately requested and unchecked on the approval page. No Fetch password or account is created.

Registrations and token-family hashes are encrypted in the application store. Refresh rotates once; reuse of a spent refresh token durably revokes that family. Standard revocation affects the entire family and emits its owner ID so the runner can stop delivery and cancel its owned work. Pending browser requests and codes last only for this process. OAuth responses retain standard formats without application `next_steps` wrappers.

The parent router supplies protected-resource metadata and the shared Host/Origin boundary. This module supplies authorization-server metadata, registration, authorization GET/POST, token and revocation routes. Native Settings session headers are recognized only if the in-process window minted a live session; they cannot be obtained through OAuth.

All new code in this pass is untested at the user's request. No app, provider, Keychain, browser, service, HTTP call or test was started.
