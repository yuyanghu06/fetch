# Publish the verified Apple Silicon preview

- Local timestamp / acting role / milestone: 2026-11-17T19:02:44-05:00 / Orchestrator / macOS preview
- Requested scope: Build Fetch with the startup icon and latest connection UI, push the source first, and publish the macOS release.
- Starting repository state: The icon and UI commits were on GitHub main. The preview was unpublished.
- Work performed / changed files: Built the final optimized arm64 binary and signed development executable. Packaged the app, dog icon and license notices. Pushed tag v0.1.0 at commit 4418d89ddab25b438ab81d1368dc057f877b3a65. Uploaded the disk image and its SHA-256 file and published the GitHub prerelease.
- Decisions: Use an explicitly ad-hoc-signed Apple Silicon preview. This Mac has no Developer ID Application identity and the preview has no Apple notarization.
- Checks and evidence: Both builds passed. The disk image mounted read-only, the app signature passed strict verification, and the bundle version, dog icon and arm64 executable matched the build. The bundled licenses match the collected set. The mounted binary's help command worked. GitHub's two asset digests matched the local files, and a public download passed the published checksum.
- Resulting state / acceptance: [Fetch v0.1.0 macOS preview](https://github.com/yuyanghu06/fetch/releases/tag/v0.1.0) published at 2026-11-18T00:01:37Z. The DMG is 19,801,450 bytes with SHA-256 `46cf214e54e07ddbebbe2dad71eeb311fe41c6fda7382b0edae8bb74305e8b98`.
- Remaining work and handoff: The minimized Dock badge still needs a live observation. Developer ID signing and notarization remain separate follow-up work.
