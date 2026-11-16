# Shared button evidence

ui-tests.txt records the final 78 passing UI checks. signed-build.txt records the final development build/signing through scripts/run-development.sh --build-only, including strict signature verification. PNGs show actual shared widgets rendered without opening Fetch: Install idle, Install feedback after a fast reply, Servers refresh hover, and Connect at the minimum window size. Raw capture buffers were converted to PNG with Python’s standard library and removed.

These fixtures do not execute their API tasks, contact MCP servers/providers, or establish live-window appearance. Review was direct, without subagents. Existing Fetch remains running its previous executable.

Follow-up: shorter-refresh-request-test.txt and shorter-refresh-feedback-test.txt record the two Servers refresh checks. inline-refresh-test.txt records discovery click/loading/retry; inline-layout-test.txt records normal/minimum screens. inline-shorter-refresh-build.txt records the final signed build (15.03s). Install PNGs now show the inline Refresh placement at both sizes.
