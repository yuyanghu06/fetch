# Protecting secrets and authorizing Settings changes

October28, 2026. **Shown direction approved in D72; unshown mechanisms remain candidates.** The user approved the commentary describing encrypted per-server files, keys held in macOS Keychain, manual secrets saved/removed only in Fetch's own native Settings window with a Mac confirmation, and no Touch ID for each tool call or automatic renewal. D26/D28 and D69–D71 remain selected. No implementation, OS/Keychain changes or live tests are authorized.

The longer draft below was not presented in full. Its Keychain mode, cipher/filenames/envelope, private capability header/bootstrap,120-second timer/error mappings and detailed recovery/signing/bridge mechanisms remain implementation/storage/onboarding candidates. D72 does not silently approve them. D73 gathers all remaining mechanisms once in build-readiness.md; do not reopen the approved broad direction or create another provider/login approval loop from this draft.

## Proposed storage and keys

Use the traditional macOS **login Keychain** for a separate random256-bit file-encryption key for each managed server. This is the Mac's existing protected key store, not a Fetch account/password. Each item has a Fetch-only trusted-application access list. Do not grant a shell, command-line security utility or arbitrary app access to these items. Exact code identity and error behavior need isolated signed-build/local-build proof.

Choose the login Keychain explicitly because the initial open-source app must support locally built/unprovisioned applications. Apple's modern data-protection Keychain is a separate option requiring provisioned signing/access-group arrangements. Do not silently switch stores or apply its accessibility/migration guarantees to the login Keychain. Local/ad-hoc rebuilds may cause macOS to ask the person to allow the changed Fetch build again; ordinary updates should preserve data/key records. Release signing/identity mechanics remain step5/7 decisions.

Propose these actual names within the selected D28 directory:

| File or state | Contents / protection |
| --- | --- |
| `servers/<server-id>/config.json` | Non-secret settings and references; no provider tokens, secret environment values or API keys. |
| `servers/<server-id>/credentials.enc` | That server's encrypted credential document, opened only with its matching Keychain key. |
| Fetch-owned app authorization state | Separate encrypted app-owned record/key, not a provider server's credential file. |
| Fetch HTTPS private identity | Separately protected identity/key; never reuse a provider file-encryption key as a TLS key. The public certificate may be stored openly. |

The paths above are under `~/Library/Application Support/Fetch/`, outside `packages/`. Exact app-owned authorization/TLS filenames and serializer/library versions remain the bounded implementation review. No cloud sync or key-export/backup product feature is selected.

Use standard **AES-256-GCM** authenticated encryption: a fresh random96-bit nonce on every write, authentication tag and version/key identifier. Bind the server ID, file purpose and format version as authenticated associated data so moving a credential file to another server cannot silently change its meaning. Use an established implementation; exact Rust dependency/version remains step5. No key sits beside its encrypted file. Authenticated encryption detects alteration; it does not prove that an older valid file has not been restored.

Backend APIs serialize writes per server, create only encrypted temporary files in the same restrictive directory, validate/save, atomically replace and read back the result. Keep directories owner-only and files owner-readable/writable. No plaintext staging, diagnostic dump or backup. Key creation succeeds/readbacks before publishing a first encrypted record; failed initialization must not leave a fake successful configuration.

Preserve D25/D31: successful encryption/readback is not the full credential-save success gate. The newly stored applicable credential still needs its required authenticated provider test before200. Verifier endpoint/failure/batch/no-op policies remain the explicit4d review.

## Proposed Settings authority on the existing API

Manual secret entry is initially available through **Fetch's own native Settings window**. D72 selects this shown initial-slice restriction. A normal browser page does not automatically receive human Settings authority; browser Settings secret entry would need its own reviewed mechanism. Ordinary browser/MCP app-connection approval remains D71.

At native-window creation, Fetch's backend creates an unpredictable temporary Settings capability in process memory and delivers it only through its private in-process connection to that native window. Its backend record carries human Settings authority and the allowed server/operation permissions, so first setup does not depend on an already-connected agent. It expires on native-window destruction or Quit. Reopening creates a fresh capability; nothing is persisted in client configuration or a file.

The native HTTP client sends that capability in `X-Fetch-Settings-Capability` **only to the existing same-origin HTTPS credential PATCH**. It is never issued/returned by public HTTP, placed in a browser URL, exposed to arbitrary pages/frames, returned to agents, inherited by child processes or logged—including HTTP request diagnostics. The native window loads trusted bundled UI; exact native bridge/frame/navigation checks are step5 implementation/evidence requirements. This does not select Iced, a webview framework or a new API route.

The same existing backend handler for `PATCH /api/v1/servers/{server_id}/credentials` checks both live capability authority and the current target's server-management permission before prompting or changing secrets. Ordinary agent OAuth/MCP credentials get403 even on direct HTTP; an extra body field claiming to be Settings cannot create authority. Tests must prove this through actual HTTP before the Settings UI, as the API-first rule requires. No direct UI file-writing path or alternative credential-save behavior is introduced.

For a manual set/remove save attempt, additionally request a fresh Mac confirmation using D71's device-owner authentication method. Bind the result to the exact server ID, requested mutation, current credential-document revision, relevant saved-config revision and still-live Settings window/capability. Reuse of an earlier confirmation cannot authorize different data. Release coordination locks while the person responds, then recheck capability, permissions, credential/config revisions and incompatible operations under backend write coordination immediately before committing. Changed state uses the existing conflict/busy behavior, never a silent overwrite or loss of automatic refresh updates. These are internal revisions, not a new public request field or history feature. Do not settle D31's no-op/provider-verification rules through this authority check.

Propose a120-second maximum Mac confirmation wait, no automatic prompt retries and no cached successful context for later saves. Denial/cancellation/expiry/window closure/Quit produces no new secret write. Invalidate the pending native confirmation and ignore late results. If a save committed before closure, report state truthfully rather than claiming it rolled back.

Metadata/status GETs retain their read-only/latest-known projection and do not unlock Keychain or read secret values just to render a page. Record safe store failures from actual attempted operations rather than claiming a fresh successful key probe on GET.

Ordinary tool calls, provider login/refresh callbacks and app-token renewal use their existing backend authority and **do not ask for Touch ID on every operation**. Trusted backend OAuth writes remain automatic; they never borrow a native Settings capability. Non-secret configuration APIs keep their approved authority and behavior.

## Unlock, failures and recovery

An unlocked login Keychain plus permitted Fetch identity supplies keys during normal operations. Background key access is noninteractive: locked Keychain, denial or unavailability stops the affected credential-dependent operation and gives a safe human handoff rather than repeatedly showing OS dialogs. A deliberate foreground action may allow the OS unlock/access prompt. Do not confuse denied access or a locked store with a genuinely missing first-use key.

Screen lock and Keychain lock are different. Locking Keychain does not retroactively erase key/plaintext copies already held in memory or values already assigned to a running child process. Avoid unnecessary long-lived key copies; no perfect memory-erasure guarantee or automatic child-stop/credential-revocation feature is selected. Quit releases Fetch's in-memory state and stops children under the existing lifecycle.

| Condition | Proposed behavior |
| --- | --- |
| Empty first-use storage | Create that server's key and encrypted record through the backend operation. Never import credentials from another client. |
| Keychain locked, unavailable or access denied | Stop the affected read/write; no plaintext fallback, new replacement key or provider-auth success. Preserve existing files and unrelated servers. |
| Existing file but key absent/wrong, bad authentication tag or unsupported format | Preserve the unreadable file; report a safe store failure. Do not treat it as empty or silently overwrite it. |
| Save/readback failure | Preserve the last committed record where possible and return the existing safe save failure. Do not claim200, run a test against unsaved data or claim rollback after a committed write. |
| Original key cannot be recovered | Offer an explicit user recovery decision: fresh authentication/re-entry into a new protected record while preserving the unreadable original. Exact recovery/reset operation/API remains separately reviewed, not a hidden settings action. |
| Copy to a new Mac | Copying Fetch's directory alone does not copy its Keychain keys. No automatic migration/export guarantee; fresh login/re-entry may be required. |
| Fetch app-token or HTTPS key loss | Preserve provider records. App-token state needs explicit recovery/reapproval; TLS key loss needs certificate replacement/trust recovery. Never reset the entire Keychain or all servers. |

API failure mapping proposal: missing/invalid Settings authority keeps403 `FORBIDDEN` before a prompt; human cancellation/denial uses403 `HUMAN_CONFIRMATION_DENIED`; confirmation expiry uses408 `HUMAN_CONFIRMATION_TIMEOUT`; unavailable interaction uses503 `HUMAN_CONFIRMATION_UNAVAILABLE`. Keychain locked/unavailable uses503 `CREDENTIAL_STORE_UNAVAILABLE`, denied key access uses403 `CREDENTIAL_STORE_ACCESS_DENIED`; missing/corrupt/unsupported existing credential records use500 `CREDENTIAL_STORE_READ_FAILED`. Preserve existing `CREDENTIALS_WRITE_FAILED` for save/readback and D31 `CREDENTIAL_VERIFICATION_FAILED` for the later provider test. Exact native OS codes are internal; never expose secret values or turn denial into item-not-found success. Application JSON keeps no-store and next_steps; UI follows structured status, never guidance text.

Settings authority/header changes and store mechanics are proposed amendments to the existing credential API, not new credential routes. The explicit recovery/reset and browser Settings authority routes remain unselected; do not implement or imply them silently. General key rotation/backup/export is not added by choosing encrypted files.

## Source evidence and remaining proof

[Apple TN3137](https://developer.apple.com/documentation/technotes/tn3137-on-mac-keychains) distinguishes login and data-protection Keychains; [SecAccessCreate](https://developer.apple.com/documentation/security/secaccesscreate(_:_:_:)) supports per-item trusted applications. [Noninteractive access](https://developer.apple.com/documentation/security/ksecuseauthenticationuifail) informs background handling. Keychain-held symmetric data is not automatically a non-exportable Secure Enclave key. [Apple key storage](https://developer.apple.com/documentation/cryptokit/storing-cryptokit-keys-in-the-keychain) and [AES.GCM sealed data](https://developer.apple.com/documentation/cryptokit/aes/gcm/sealedbox) inform the separation/encryption proposal.

The maintained MIT/Apache-2.0 [apple-native-keyring-store](https://github.com/open-source-cooperative/apple-native-keyring-store) explicitly separates unprovisioned login-Keychain and provisioned protected modes. Its [open denial/error issue43](https://github.com/open-source-cooperative/apple-native-keyring-store/issues/43) and [user-presence discussion25](https://github.com/open-source-cooperative/apple-native-keyring-store/issues/25), plus closed rebuild-related issues, justify actual error/rebuild/prompt tests; these are not reproduced Fetch bugs. The inspected provider uses security-framework/keyring-core; no dependency is selected.

Maintained MIT/Apache-2.0 [RustCrypto aes-gcm](https://github.com/RustCrypto/AEADs/tree/master/aes-gcm) is a candidate for the Rust backend; its historical2020 NCC audit is not an audit of current Fetch or every later version. Apple CryptoKit is an alternative with a native/Swift bridge. Dependency/version/envelope selection remains step5, with no copied implementation or installed runtime.

After explicit authorization, D46 isolation must prove signed/local-build access lists; locked/denied/missing-key distinctions; no background prompt loops; encrypted-file tamper/server-swap rejection; first-write/atomic-save/readback failures; actual HTTP positive native-capability saves and negative OAuth/header/body spoofing before UI; fresh Mac confirmation and window/revision/late-result races; trusted automatic renewal without manual-authority leakage; D25/D31 success gate; and recovery preserving old/unrelated records. No Keychain, certificate, credential, client, app service or environment was changed or tested for this proposal.
