# Installation HTTPS identity

Fetch creates one localhost TLS certificate and private key per installation. The private identity is encrypted in the separate application store and uses its own Keychain-held file key. Reopening loads the saved identity; an unreadable existing identity is preserved and blocks startup rather than being replaced silently.

This source never installs trust or disables certificate verification. Certificate trust and real client behavior await user-started testing.
