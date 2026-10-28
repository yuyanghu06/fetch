# GitHub through GitHub CLI — proposal for review

Current D73 checkpoint: this custom Fetch-owned adapter is the one remaining product-scope choice. Recommendation is to defer it; that recommendation is not yet selected. Importing an existing GitHub MCP remains a distinct source-specific compatibility task. Do not treat the full draft or recommendation as approved.

October 27, 2026. The user proposed guiding users to install GitHub CLI and exposing CLI-backed GitHub tools, instead of providing explicit GitHub MCP authentication support. This document makes that proposal reviewable; exact integration, commands, installation ownership and authentication mechanism are not yet selected. General auth API behavior is accepted under D27, per-server encrypted credential files under D26 and stored-credential testing under D25. Implementation remains paused.

## Proposed product wording and boundary

Recommend calling this **GitHub via GitHub CLI**, while saying that the official GitHub MCP server's automatic OAuth setup is not a first-class supported integration. Saying GitHub itself is unsupported would conflict with offering GitHub tools. Explicitly excluding a first-class integration need not prohibit installing an arbitrary MCP package; a blanket package ban was not selected.

`gh` is a CLI, not a built-in general MCP server in the version researched. Fetch still needs an adapter mapping selected named tools to validated application API operations and then `gh` argument arrays. Its MCP/UI clients keep using the same application APIs. This does not choose those routes/tool names or add arbitrary shell access.

## Bounded implementation recommendation

Expose a reviewed set of operations with typed inputs and controlled output, such as repository/issue/pull-request operations only if selected later. Do not expose an unrestricted command string, arbitrary extensions or unbounded `gh api` flags. Secret-returning commands (`gh auth token`, `gh auth status --show-token`) must never become agent tools. A trusted backend integration privately obtaining a fresh token is a separate mechanism requiring design; it is not permission to import the user's existing login.

## Authentication and storage gap

The user's encrypted per-server files remain required. `gh auth login` normally uses the OS credential store and can fall back to plaintext, which does not automatically meet that rule. `GH_CONFIG_DIR` changes config paths but does not create an independent host/user keyring namespace. Do not overwrite or migrate existing personal login state.

A possible operations path is Fetch reads the assigned encrypted file, supplies only that credential to a controlled `gh` process using `GH_TOKEN`, and verifies the actual HTTP status from an authenticated `GET /user` (for example via `gh api --include`). A zero CLI exit status alone is not D25 verification. This test endpoint/CLI use is proposed, not selected for the user.

There remains a material renewal issue: `gh` does not refresh environment-supplied access tokens. Capturing `gh auth token` copies only the access token, not its whole refresh lifecycle. A fresh-login capture alone therefore does not finish persistent auth support. Two designs need user review:

- Keep the selected Fetch-owned encrypted files and define Fetch-owned provider OAuth/renewal feeding `gh` as an operations backend. This needs one explicit GitHub auth integration.
- Let `gh` own login/storage/renewal, which would require a deliberate amendment to the current encrypted-file storage policy plus safe handling of its fallback/isolation behavior. This is not silently adopted.

No token-capture, plaintext temporary cache, private app registration, refresh hook or key mechanism is chosen here. See [the source comparison](research.md#github-cli-as-an-operations-backend--october-27).

## Installation and draft help prompt

GitHub officially documents macOS Homebrew and release-binary installation. User-managed global `gh` versus a pinned Fetch-private copy is an open choice. The following is an **installation-only draft**, not an executed command or complete login guide. Login instructions must follow the selected per-server auth/storage integration rather than creating an unmanaged credential copy by default.

```text
Help me install GitHub CLI on this Mac using GitHub's official macOS instructions:
https://github.com/cli/cli/blob/v2.103.3/docs/install_macos.md

First check whether gh is already installed and report its version and location.
If it is missing, explain the official Homebrew and release-binary options so I
can choose the installation method. Verify gh --version after installation.

Stop before authentication. Fetch's login flow must be configured to save this
integration's credentials in its own encrypted configuration file and verify
an authenticated request using the credential read back from that file.
Do not print tokens, run gh auth token or --show-token, migrate an existing login,
or change Fetch or assistant configuration as part of installing the CLI.
```

## Decisions and verification still needed

Choose whether to adopt this integration, its named operations, app API mappings, installation/version ownership, fresh-login/refresh method and exact provider test. D28 now selects [durable server config outside replaceable packages](storage-scope.md#approved-server-data-placement--d28); filenames/encoding/key protection remain open. This does not choose the CLI integration. Verify no secret output, controlled arguments, personal-login isolation, encrypted store/readback/actual HTTP `2xx`, renewal/rotation, restart and installer lifecycle before claiming support. No such test has run.
