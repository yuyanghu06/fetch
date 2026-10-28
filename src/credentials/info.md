# Encrypted server credentials

This module saves and reads server-local encrypted bytes using AES-256-GCM with fresh random nonces. The authenticated data binds the file format and exact server ID, so copying ciphertext between servers fails verification even if a test key is shared. Corrupt data and missing keys are not replaced. Secret documents clear their owned buffers on drop and have a redacted debug representation.

Keys come from an injected KeyProvider. This first increment has no real Keychain implementation and no plaintext-key fallback. Only tests supply fixture keys. No HTTP endpoint exposes secret bytes. Successful encrypted write/readback proves storage only; the separate required authenticated provider test is still needed before any API can report login or credential-save success.
