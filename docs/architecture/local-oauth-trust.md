# Local OAuth HTTPS and certificate trust

October28, 2026. **Presented HTTPS/certificate direction approved in D70.** After the explanation of certificate creation and browser/client verification effort, the user said "Good to go for this part, next." The local HTTPS listener, unique per-install certificate/key and explicit trust/normal-verification test direction are selected. Exact key/certificate lifecycle, client mechanics and actual isolated proof remain open. Application implementation and certificate/client/trust-store/environment changes remain paused.

## Approved account-free direction — D69

The user explicitly selected no Fetch account, signup or password. Apps use a browser connection to local Fetch to obtain their own access token, keeping the selected standard exchange/renewal direction. The browser is a way to connect the app, not a new Fetch account system. Keep the experience simple because Fetch is local.

The earlier phrase "Fetch's browser login" was too broad. No specific browser screen, separate account service or certificate setup was approved by D10. D69 confirms the account-free browser direction; D70 subsequently selects the presented HTTPS/certificate direction. Exact browser approval/authority controls and token/callback contracts remain open.

## Approved direction — D70

Use one HTTPS loopback listener in the Fetch process for the existing `/mcp` resource, application APIs and Fetch's OAuth issuer/endpoints/metadata. Fetch remains accessible only on this computer. D70 replaces D47's candidate HTTP resource scheme with HTTPS; it adds no public listener, hosted identity service or second gateway process. Exact port and published loopback hostname/address remain for the bounded listener/onboarding review; endpoint contracts use that stable published origin rather than inventing a second listener.

Generate a unique server certificate and private key for each Fetch installation. Prefer trust limited to that Fetch certificate and its local hostname, with explicit human setup in the browser/OS and each supported client where necessary. Do not distribute a shared private key, install mkcert as an end-user dependency, disable certificate verification or silently trust a broader local certificate authority. Exact key storage and access protection remain part of the already pending key-management review; this proposal does not select a plaintext key file.

The proposed first-run flow is: explain the exact certificate/trust change; have the user perform or explicitly authorize it through the proper system/client control; then check the connection using the selected isolated acceptance plan. Ordinary agent grants cannot authorize trust-store changes. Browser trust does not prove that Claude Code, Codex or OpenCode trusts the connection, or the reverse.

If trust is missing or a client rejects the certificate, connection/setup reports that prerequisite and stops before claiming login success. There is no HTTP or disabled-verification fallback. Replacing the trusted server certificate requires explicit trust setup for its replacement. The login flow, user permissions and provider credentials still follow their selected backend contracts; an HTTPS certificate is not a Fetch login credential.

If isolated evidence shows that certificate/hostname-limited trust cannot support a required everyday client/browser, bring back that concrete finding and the wider-local-CA alternative for the user's decision. Do not silently widen trust or mark the required client compatible. Removing trust on uninstall and handling lost keys/certificate replacement are dependencies of the key/onboarding/release contracts; no cleanup action is authorized here.

## What the choice costs

This keeps Fetch locally runnable without the original developer's domain or identity service, but adds certificate setup and possible per-client configuration. Narrow trust is a design target, not established compatibility. A self-signed certificate's `CA:false` flag alone does not prove every TLS stack will restrict it to one host or prevent its use as a broader trust anchor.

### Effort estimate and what a successful test proves — October28

The user asked how hard issuing a certificate and verifying it would be. Root's engineering estimate, not measured evidence: creating a correctly named local certificate/key takes minutes; a browser plus one-client HTTPS/trust proof should take a few hours once an isolated Mac test environment and minimal HTTPS test service are ready. Allow roughly one to two days for the first pass across Claude Code, Codex and OpenCode, including setup differences, relaunch/reconnect and troubleshooting. An incompatible certificate-trust path could take longer. These estimates exclude provisioning the isolated environment and implementing Fetch's still-unbuilt authorization backend.

A certificate-only experiment can prove that the browser and clients accept the intended local address/certificate with normal verification enabled, and reject an unrelated/untrusted or wrong-address certificate. It cannot prove the full browser approval/token flow. Once an appropriate disposable test authorization service exists, separately verify that the app obtains its token, lists tools, renews access and reconnects after restart. Current client trust documentation supports trying this; it is not a passed test.

Creating the certificate is routine work; supported-client trust behavior is the main uncertainty. Making certificate setup, expiry/replacement and cleanup easy for every user is additional product work. This estimate neither approves the HTTPS proposal nor authorizes certificate creation, trust-store changes, test environment provisioning or live MCP tests. D69's no-account/browser-token direction and the existing implementation pause remain unchanged.

| Alternative | Benefit | Tradeoff / status |
| --- | --- | --- |
| Unique Fetch server certificate with demonstrated certificate/hostname restrictions | Preferred narrow trust target; no central service. | Must prove the actual restrictions and acceptance in required browsers/clients; replacement needs new trust. Proposed default. |
| Per-install local CA issuing Fetch certificates | Easier leaf-certificate renewal. | Gives the CA broader signing authority and adds key/trust protection work. Requires a separate explicit choice if needed. |
| Operator-provided public-domain certificate | Uses existing public certificate trust where valid. | Requires a domain, issuance and renewal setup; no such service or requirement is selected. |
| Plain HTTP OAuth | Less certificate setup. | Does not satisfy the cited HTTPS authorization-server requirements; would require an explicit standards-policy reconsideration, not a compatibility shortcut. |

## Evidence and limits

### How other MCPs approach this — October28 research

Local certificates are one possible design, not a requirement for every MCP. The MCP connection and the service issuing login tokens are separate parts, even when Fetch proposes to put them in the same process/listener.

| Pattern | Actual example | What a person sets up |
| --- | --- | --- |
| Client starts a local program and exchanges messages directly through program input/output. | [Official Filesystem server](https://github.com/modelcontextprotocol/servers/blob/main/src/filesystem/README.md). | No HTTP connection or local TLS certificate for this hop. This is stdio, not the D47 client-facing transport selected for Fetch. |
| A local HTTP address. | [ToolHive embedded authorization configuration](https://github.com/stacklok/toolhive/blob/main/pkg/authserver/config.go). | Its validator permits an HTTP localhost issuer; no local certificate. This is its deliberate local exception, not evidence that the cited OAuth HTTPS requirements disappear. |
| A hosted HTTPS MCP. | [GitHub remote MCP](https://github.com/github/github-mcp-server#remote-github-mcp-server). | The operator supplies a normal publicly trusted server certificate; users ordinarily install no custom certificate. This does not select hosting for Fetch. |
| Local HTTPS for the authorization service. | [Official C# protected-client sample](https://github.com/modelcontextprotocol/csharp-sdk/blob/main/samples/ProtectedMcpClient/README.md). | The developer trusts a .NET development certificate for its HTTPS authorization server. The sample's MCP resource uses HTTP. This is a development example, not proof of complete Fetch metadata/client compliance or an end-user installer design. |

[MCP local transport rules](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http#security--endpoint) permit local HTTP with appropriate local binding/security. The selected authorization-server/issuer/metadata requirements still need separate treatment. Do not infer that a localhost redirect exception permits plaintext token endpoints, or that generating a certificate proves trust/compatibility.

The Researcher inspected these actual MCP examples and reused prior maintenance/license/dependency/issue checks for ToolHive. GitHub MCP is MIT and active at inspection. Official MCP repositories' contribution/license transitions and documentation licenses should be checked per file before copying; no example code/dependency is adopted. The C# sample references the official core SDK and .NET9; its development certificate is not a production Fetch choice. No service, client or certificate was tested.

The comparison qualified the earlier recommendation before D70: local HTTPS has setup cost; ToolHive-style local HTTP relaxes the strict scheme requirements. D70 now selects the presented HTTPS/certificate direction, so these alternatives are reference material rather than an active scheme review. Do not replace the selected browser token flow with a static token, add a stdio bridge or install trust entries without the applicable decisions/authorization.

### What setup would look like — clarification, October28

Under D70, Fetch will create its own certificate and matching secret key, configure its local listener to use them, then guide the person through trusting the certificate once implementation is authorized. The certificate must identify the exact local hostname/address clients use. Making a certificate for `localhost` does not automatically cover a connection made to a numeric address. Exact names, key storage and replacement remain in the pending design.

On macOS the human procedure is to add the certificate to Keychain Access, open that certificate, expand Trust and set its appropriate connection trust policy. Apple documents [creating](https://support.apple.com/guide/keychain-access/create-self-signed-certificates-kyca8916/mac), [adding](https://support.apple.com/guide/keychain-access/add-certificates-to-a-keychain-kyca2431/mac) and [changing trust](https://support.apple.com/guide/keychain-access/change-the-trust-settings-of-a-certificate-kyca11871/mac). This is an explanation, not an instruction to change the current machine or a choice of broad Always Trust defaults. Application code could generate/configure its certificate, but OS trust still needs the selected human authority and actual client verification.

Current [Claude Code documentation](https://code.claude.com/docs/en/network-config#ca-certificate-store) says its default combines bundled certificates and the operating system's trust store. The native installer supports the necessary runtime; npm installations need Node22.15 or later for this behavior. Older runtime installations may need its documented extra-certificate setting and relaunch. Do not apply generic Node defaults to all Claude Code installations. These are documented CA-store capabilities, not proof that the proposed narrowly trusted Fetch server certificate works in MCP OAuth or every browser.

Other clients may use different certificate stores or explicit certificate-file settings. [Node's trust documentation](https://nodejs.org/learn/http/enterprise-network-configuration#certificate-authority-configuration) provides examples; it does not establish Codex/OpenCode behavior. After setup, independently test the browser login and each required client against the real certificate/address, keeping certificate validation enabled. No such tests are authorized or performed by this clarification.

Both selected MCP revisions require HTTPS authorization-server endpoints; the loopback HTTP exception is for redirect URIs. [Current communication security](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization/security-considerations#communication-security), [legacy authorization](https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization#communication-security), [RFC8414 issuer requirements](https://www.rfc-editor.org/rfc/rfc8414.html#section-2) and [RFC9728 metadata TLS](https://www.rfc-editor.org/rfc/rfc9728.html#section-7.3) constrain the issuer/metadata arrangement. An HTTP MCP resource with a separate HTTPS issuer does not by itself settle all metadata trust.

Apple provides [certificate trust settings](https://developer.apple.com/documentation/security/sectrustsettingssettrustsettings%28_%3A_%3A_%3A%29) and [usage constraints](https://developer.apple.com/documentation/security/usage-constraints-dictionary-keys), including SSL hostname restrictions, with user authentication. This is evidence for a possible narrow design, not proof across TLS implementations. [Firefox's Apple trust import](https://github.com/mozilla-firefox/firefox/blob/main/security/manager/ssl/EnterpriseRoots.cpp) rejects records with application/hostname constraints. [Node's extra/system CA settings](https://nodejs.org/api/cli.html#node_extra_ca_certsfile) illustrate separate runtime trust; no environment change is selected.

[Let's Encrypt's localhost guidance](https://letsencrypt.org/docs/certificates-for-localhost/) explains why public localhost certificates and shipping a common certificate/private key are unsuitable defaults. Public-domain per-install issuance adds infrastructure rather than eliminating certificate management.

Read-only comparisons: [Caddy](https://github.com/caddyserver/caddy) is maintained, Apache-2.0 and uses CertMagic/Smallstep for its [local CA design](https://caddyserver.com/docs/automatic-https#local-https); it warns that trust installation may need manual steps. [ToolHive](https://github.com/stacklok/toolhive) is maintained, Apache-2.0 and uses Ory Fosite in its [embedded auth server](https://github.com/stacklok/toolhive/blob/main/pkg/authserver/config.go); its localhost-HTTP policy is an implementation choice, not an exception to MCP's normative HTTPS requirements. Its [CA/config propagation issue6522](https://github.com/stacklok/toolhive/issues/6522) was closed October23 and is a historical testing lesson, not a current defect claim. [mkcert](https://github.com/FiloSottile/mkcert) is BSD-3-Clause, last pushed August2024 at inspection, and explicitly targets development rather than end-user installations. These comparisons select no dependency.

The Researcher inspected these public sources on October28. No certificate, private key, trust change, client edit, service, test environment or live OAuth/MCP connection was created. Browser/client versions, hostname/chain restrictions, renewal/replacement and complete setup need authorized isolated evidence before compatibility is claimed. The [remaining checklist](open-questions.md#remaining-decisions-checklist) keeps dependent OAuth, Settings/key, onboarding and test/release work open.
