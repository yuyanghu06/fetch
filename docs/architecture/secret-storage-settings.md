# Saving keys and authorizing Settings changes

November3,2026. [Current complete contract](native-server-storage.md).

Local servers keep provider credentials in their native files or supported OS store.
Fetch does not promise those files are encrypted. The old provider encrypted files,
keys and runtime state remain untouched; their readers/writers, token interception
and private bridge are removed from current source.

Fetch stores only its online-client OAuth grants and manually entered fixed keys
in a shared direct macOS Keychain connection, separated by Fetch root and instance.
The official library controls online exchange/renewal/replacement. SDK and manual
sections preserve each other; successful writes require readback. There is no
plaintext fallback or automatic legacy migration/decryption/deletion.

PATCH /api/v1/servers/{id}/credentials remains native Settings only. A live private
window capability and fresh, exact request-bound Mac confirmation are required.
Ordinary agent/browser OAuth credentials cannot write manual keys. Existing assigned
environment/header names are the allowed destinations; values never appear in API
or MCP results. Return stored, verified:false, affected names, restart_required and
next_steps. Storage alone does not prove the service accepts the key.

Fetch's own TLS identity and coding-app authorizations retain their existing
protected application files and keys. That separate store refuses provider IDs
before accessing any file or key. /oauth/token and caller protections remain.
The private memory-only background credential authorizes quiet authenticate only;
it cannot save keys, run setup, open a browser/Finder or invoke tools.

Actual native Keychain, Mac approval and Finder acceptance remain unrun. Disposable
fake-store tests do not establish these OS behaviors or provider-account access.

Earlier decisions and implementation evidence remain in [the historical record](../../milestones/native-server-storage/historical-secret-storage-settings.md); conflicting encrypted-provider requirements are superseded.
