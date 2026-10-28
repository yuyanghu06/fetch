# Encrypted server credentials

This module saves and reads server-local encrypted bytes using AES-256-GCM with fresh random nonces. The authenticated data binds the file format and exact server ID, so copying ciphertext between servers fails verification even if a test key is shared. Corrupt data and missing keys are not replaced. Secret documents clear their owned buffers on drop and have a redacted debug representation.

Keys come from an injected KeyProvider. This first increment has no real Keychain implementation and no plaintext-key fallback. Only tests supply fixture keys. No HTTP endpoint exposes secret bytes. Successful encrypted write/readback proves storage only; the separate required authenticated provider test is still needed before any API can report login or credential-save success.

The production `MacKeychainKeys` provider uses per-server random keys in the traditional Mac Keychain. It limits each key to the running Fetch application and fails noninteractively when access is unavailable. Reading or replacing an existing encrypted file requires its existing key; a missing key is never silently replaced. App authorization and TLS identity use a separate encrypted application tree. No Keychain or provider operation was executed while writing this source.
