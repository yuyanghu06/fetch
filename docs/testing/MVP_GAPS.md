# Selected MVP work that is still incomplete

October 28, 2026. The user reaffirmed that the MVP must actually run the selected
product. Compiling, importing a server or displaying an action is not completion.
This is a source audit, not a runtime test result.

| Gap | Source evidence and remaining work |
| --- | --- |
| Fresh provider setup across everyday imports | Manager supports only a small provider list. Railway browser setup is implemented in source and compiles; broader selected Google/Slack/other integration ownership and verification are unfinished. An unknown result is truthful error handling, not completion of required setup. |
| General remote OAuth | Transport turns 401/403 into an auth error. Protected-resource/issuer discovery, dynamic registration and refresh remain integration-specific. Railway-specific support does not complete this general path. |
| Advertised credential operations | Current save verification only accepts one recognized effective auth entry with no OAuth token. Other environment/custom header entries, batch updates and removal-only changes can commit then fail verification. Their successful workflows need implementation consistent with the approved stored-credential verification contract. |
| Import compatibility | Existing real definitions include launcher arguments and URL forms excluded by current classifier. Resolve supported safe mapping against the recorded inventory rather than silently declaring all excluded inputs deferred. |
| Runtime acceptance | Native/browser/Keychain/TLS/provider/client/queue/lifecycle checks are still unrun. No working-full-MVP claim is supported. |

Current fixes address inline auth presentation, independent pending-operation
polling, first-party HTTP diagnostics and Railway browser/runtime preparation.
These fixes require their own source/build evidence; live results remain separate.

Railway's [documented native callback](https://docs.railway.com/integrations/oauth/quickstart)
uses HTTP on 127.0.0.1 or an app URL scheme. D80 selects the temporary numeric-loopback
callback listener; it is now implemented and source-reviewed. No live login has
been run. The existing product API stays on HTTPS.
Fetch must own its fresh registration/tokens and verify encrypted readback through
[/oauth/me](https://docs.railway.com/integrations/oauth/login-and-tokens).
The local CLI inherits an environment token; after background token refresh the
running process needs explicit Restart to receive the new token. This limitation
must remain visible until a reviewed mechanism can apply rotation safely.

Explicitly deferred custom GitHub CLI tools, memory, PostgreSQL/retention, graph,
extra processes, native background Tasks and live resource subscriptions retain
their earlier status. This audit does not add those to the initial release.
