# Public client identity adoption in MCP

Researched November 4, 2026 from primary sources. This is research, not a new implementation decision or a live compatibility test.

A Client ID Metadata Document (CIMD) is a public JSON file describing the connecting application and its permitted sign-in return addresses. The application uses the file's HTTPS URL as its client identifier. The authorization service reads the file. It is not a user's credentials, model prompt content or independent certification of the application.

## Findings

This is the standard direction for remote MCP browser sign-in, with uneven deployment. The [MCP July 28, 2026 release](https://blog.modelcontextprotocol.io/posts/2026-07-28/) formally deprecated dynamic client registration in favor of CIMD while retaining compatibility. The [official authorization specification](https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization#client-registration-approaches) recommends using an existing registered identity first, then CIMD when advertised, then older dynamic registration where supported.

| Service | Confirmed by its own documentation |
| --- | --- |
| Vercel | [Added CIMD November 2, 2026](https://vercel.com/changelog/vercel-mcp-now-supports-client-id-metadata-documents). Previously reviewed clients remain supported. Unreviewed clients show their publishing hostname and do not get Vercel's verified badge. |
| monday.com | [Supports both methods and recommends CIMD](https://developer.monday.com/api-reference/docs/mcp-dynamic-client-registration). Additional connections still require account-admin approval. |
| Sumo Logic | [Recommends CIMD](https://www.sumologic.com/help/docs/api/mcp-server/), but an administrator must enable it. Organizations can restrict permitted clients. |
| Linear | [Documents dynamic registration](https://linear.app/docs/mcp). This establishes its documented flow, not proof that CIMD is unsupported. |

The official [Python SDK guide](https://github.com/modelcontextprotocol/python-sdk/blob/main/docs/client/oauth-clients.md#client-id-metadata-documents) supports CIMD but describes server adoption as incomplete. That statement is qualitative, not a measured market-wide survey. The inspected rmcp 3.8.0 implementation already chooses CIMD only when advertised and otherwise keeps its supported registration path. SDK support must not be counted as proof that every hosted service accepts the feature.

Local command-based MCP servers and connections using supplied API keys do not generally use this remote OAuth client registration process. Server or organization policy may still require approval even when CIMD is supported.

## Meaning for Fetch

The same public Fetch identity file can be reused with other supporting services. It is reusable infrastructure, not a separate file to invent for each provider. Each provider still issues its own account access and can require its own consent or administrator approval. No credentials or account access are shared merely by reusing the public file.

Keep library-owned negotiation and existing token/account separation. A public file alone does not make Fetch trusted or approved. Moving its URL later changes the client identity and may require new authorization. This research establishes documented support only; no provider accounts, live metadata probes or sign-ins were used.


## Figma registration rejection — November 4, 2026

The user supplied initialize HTTP 401, discovery HTTP 405 then two HTTP 200 responses, and dynamic registration HTTP 403 with a plain Forbidden body. Registration, before browser authorization, is the blocking stage. The 405 did not stop this attempt from reaching registration. These logs do not disclose Figma's exact rejection rule.

Fetch's `src/remote_oauth/sdk.rs` supplies `client_name: Fetch` and the existing public identity URL. `src/manager/remote_callback.rs` creates and supplies the same temporary `http://127.0.0.1:<port>/oauth/provider/callback` address; source inspection found no callback/listener mismatch. This is source evidence, not proof of the running binary's exact request or Figma's acceptance of that callback.

[Figma staff Jaycee Lewis, May 1, 2026](https://forum.figma.com/report-a-problem-6/figma-s-approach-breaks-the-core-promise-of-mcp-52507) explicitly confirmed that dynamic registration checks client_name against an approved list. [Current Figma docs](https://developers.figma.com/docs/figma-mcp-server/) list supported clients and direct developers of new clients to the [waitlist](https://form.asana.com/?d=10497086658021&k=kBG-ejRQTdY8x_H6a4vM3Q); Fetch is not listed. OpenCode is now listed, so the historical post's OpenCode rejection must not be presented as its current status.

Read-only inspection of [Figma's public authorization metadata](https://api.figma.com/.well-known/oauth-authorization-server) returned registration_endpoint `https://api.figma.com/v1/oauth/mcp/register` and scope `mcp:connect`, with no `client_id_metadata_document_supported` field. Inspected rmcp 3.8.0 source (`transport/auth.rs`, AuthorizationSession::new and register_client) uses the public identity URL only when that support flag is true; otherwise it falls back to dynamic registration using Fetch's name and callback. The public Fetch JSON therefore does not bypass Figma's approval policy.

Conclusion: rejection of an unapproved Fetch client is strongly supported; a callback rejection is not established. The bare 403 prevents conclusive attribution of the internal rule. Recommend requesting official Fetch approval through the current waitlist, or using an already supported client in the meantime. Callback changes or another client's name are not selected. No registration POST, MCP connection, account login, app launch, credentials/config edits, builds or tests were performed. Independent read-only Researcher agrees with this diagnosis.

Subsequent user selection, November 4: mark Figma Remote unsupported in Fetch’s import/install screen with the approval reason. Preserve Desktop and existing installed entries. This replaces the recommendation to pursue approval as the active task; no approval request was sent. [Implementation status](../../milestones/figma-remote-unsupported/info.md).
