The local connector uses the existing signed development launcher/build-only flow; no new install, client configuration mutation or release script is introduced. See ../milestones/local-mcp-connector/evidence.md.

# Developer checks

Run `./scripts/verify-dev.sh` from the checkout to check formatting, compiler warnings and the fixture/backend HTTPS tests. The script does not start installed MCP servers or change client, Keychain or certificate-trust settings.

package-macos.sh constructs an app/DMG only when explicitly enabled, from an already-built binary; it does not build/test/publish. D77 ran neither this script nor the existing foundation verification script.

run-development.sh is the normal stable-identity DEVELOPMENT launcher. It builds
Fetch, signs and verifies a staging copy using an explicitly configured public
certificate fingerprint and fixed identifier, atomically replaces fetch-development,
then launches it. --build-only stops before launch. It never creates/exports a
private key, changes trust, falls back to ad-hoc signing, or modifies release
packaging. The selected private signing key stays in login Keychain.

Approved icon source: package-macos.sh copies packaging/Fetch.icns into bundle Resources; Info.plist names that file. This source update does not enable or run packaging.


setup-session-readers.sh explicitly prepares both official readers under a supplied absolute --root; the two setup scripts own pinned source/package installation. Conversation opening never installs dependencies.
