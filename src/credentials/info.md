# Protected Fetch application state

This module now protects only Fetch's HTTPS identity and the grants for coding
apps connecting to Fetch. Existing application file paths/keys/formats remain so
this refactor does not migrate them. AES-GCM integrity checks and Keychain key
protection remain for those two application purposes.

Provider IDs are rejected before reading files or looking up keys. There is no
provider credential-file API, provider-key creation/deletion path, Google token
interception, helper socket or provider-file migration/decryption/deletion here.
Old server credentials.enc files and their keys are left untouched.

The key provider only accepts fetch-internal:tls-identity and
fetch-internal:app-authorization. Successful keys are cached in zeroizing memory
until Quit. Missing/corrupt application state fails without replacement or a
plaintext fallback. The source fixture tests use memory key backends, not a real
Keychain.

Online MCP client grants and manually entered fixed keys are stored directly in
Keychain through remote_oauth's one shared store, using the official library for
online login/renewal. Native servers manage their own files and refresh behavior.
