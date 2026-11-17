# Build Fetch for the first Apple Silicon preview

- Local timestamp / acting role / milestone: 2026-11-17T18:50:48-05:00 / Orchestrator / macOS preview release
- Requested scope: Fix the missing and late dog icon, build Fetch, push the source and publish a macOS release on GitHub.
- Starting repository state: Fetch set the Dock icon after its first window opened. The repository had no release tag, and Cargo and the bundle identified version 0.1.0.
- Work performed / changed files: Moved native icon assignment before host startup and window creation. Updated the existing package script to accept resolved notices and an explicit ad-hoc preview signature. Cleared stale generated Rust objects to free build space.
- Decisions: Preserve macOS's system thumbnail and badge. Use the existing dog ICNS in the app bundle. The available signing identities do not include Developer ID Application, so prepare an Apple Silicon preview without Apple notarization.
- Checks and evidence: Two isolated AppKit icon fixtures passed. Both the optimized arm64 build and signed development build passed. Checked the Info.plist, packaging shell syntax and source diff.
- Resulting state / acceptance: UI and icon changes are ready to push. The optimized binary is ready for bundling.
- Remaining work and handoff: Collect notices, assemble and verify the disk image, and publish the preview from the pushed commit.
