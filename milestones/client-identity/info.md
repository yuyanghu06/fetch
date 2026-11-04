# Public Fetch identity and local sign-in callback

Goal: let remote services identify Fetch through a public JSON file and return browser sign-in to a temporary address on the user's Mac.

The public file is https://www.yuyang-hu.org/oauth-client.json . It is hosted on the user's existing personal site. The www address serves the file directly; the bare domain redirects there. The document registers http://127.0.0.1/oauth/provider/callback . The native OAuth standard permits the actual callback to use a temporary port.

For a new interactive remote sign-in, Fetch opens a listener at 127.0.0.1 on a system-selected port and gives that exact HTTP callback URL to rmcp. The listener accepts only the callback route and the pending sign-in's state. The library still handles authentication. The listener closes after completion, failure, the existing ten-minute deadline or Quit. Saved-login renewal creates no listener. Fetch's main HTTPS API, certificate and existing callback route remain unchanged.

Status: complete. The public file returns HTTP 200 and JSON, and the homepage still returns HTTP 200. One new full sign-in test and two existing regression tests pass. The native build passes and the independent evaluator accepts the change. No real Vercel account sign-in or Fetch app launch was performed. See [evidence](evidence.md).

Next user action: explicitly Quit Fetch, reopen with ./scripts/run-development.sh, then retry Vercel. Moving the public identity to a dedicated domain later may require signing in again.

[Adoption research](../../docs/architecture/client-identity-adoption.md) confirms this public identity is reusable across supporting services, with separate account consent and uneven provider adoption. It adds no implementation scope.

November 4 Figma follow-up: the published identity and matching loopback callback do not establish provider approval. Figma currently advertises older dynamic registration and staff describe an approved client-name list; reported Fetch registration403 likely reflects this policy. [Diagnosis](../../docs/architecture/client-identity-adoption.md#figma-registration-rejection--november-4-2026). No source change or live sign-in.
