# macOS distribution source

Info.plist describes the one-process native application. scripts/package-macos.sh takes an already-built executable and creates a drag-and-drop DMG with Fetch.app and an Applications shortcut. It requires explicit FETCH_ALLOW_PACKAGING=1 and a fresh output destination. It never builds, tests, notarizes, publishes, installs trust or changes client configurations. An explicit signing identity is optional input for a later authorized packaging run; no signing credentials are stored here.

Packaging has not been executed. A signed/notarized production build and final notices remain acceptance work after dependency resolution and user-started verification.
