# Developer checks

Run `./scripts/verify-dev.sh` from the checkout to check formatting, compiler warnings and the fixture/backend HTTPS tests. The script does not start installed MCP servers or change client, Keychain or certificate-trust settings.

package-macos.sh constructs an app/DMG only when explicitly enabled, from an already-built binary; it does not build/test/publish. D77 ran neither this script nor the existing foundation verification script.
