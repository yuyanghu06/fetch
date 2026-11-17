## November 13 — app-local scroll phases

The bridge observes scroll events only for Fetch windows passed to configure_window. It leaves NSEvents unchanged and queues finger/momentum phase bits in event order for the native UI root wrapper. A new finger Began relatches the scroll target; momentum keeps ownership. fetch_take_scroll_phase is an in-process bridge function, not an API operation. No global event tap or input-monitoring permission is used. tests/native_scroll_phases.m exercises ordering without installing the monitor or opening a window; actual physical trackpad acceptance remains manual.

## November 12 — prepared file picker

The bridge keeps separate cached and active NSOpenPanel references. A delayed main-thread preparation after window opening allocates the hidden panel without presenting it. Choose reuses the object, preserving allowed files/folders, home-directory starting point, native sheet callback and cancellation ordering. tests/native_file_picker.m verifies hidden reuse only; live presentation latency has not been measured.

# Native human authority

NativeAuthority creates temporary sessions only for Fetch's own native Settings
window. Bearer tokens cannot manufacture these sessions. Closing the window
invalidates authority even if a request still holds its HTTP client.

Manual fixed-key changes retain fresh Mac confirmation. A SettingsCapability binds
the live session, selected server and mutation fingerprint, expires after thirty
seconds, and can be consumed once. Passwords remain in macOS. Storage success is
separate from account authentication; the backend uses direct Keychain storage for
approved manual keys. Open server folder needs a live native session but does not
request secret-save confirmation.

The Objective-C bridge retains LocalAuthentication and application Keychain
functions. Remote-client grant read/write support is shared OS storage work, not
server-specific provider interception. Application approval and HTTPS identity
remain protected. No native Keychain operation or Mac prompt is exercised by the
window-free UI fixtures; native behavior needs the approved protected session.

bridge.m also hosts the config file/folder picker and native application menu.
The picker runs on the main queue, shows hidden locations and returns only a path
or cancellation. Backend discovery reads the selected known files later. Picker
selection does not create authentication authority or change server configuration.

## Native window appearance

configure_window accepts only the exact view borrowed through Iced's window callback, on the main UI thread. The bridge leaves it borderless/transparent and disables the redundant system shadow. Actual AppKit standard Close/Minimize/Zoom buttons are anchored top-left (24px left,18px top) and resize with that view. Close uses the existing window delegate because borderless performClose has no built-in titlebar widget; winit/Iced still decides closure and invalidates native sessions through its existing events. No NSApplication delegate replacement, other-window scan or authentication-policy change. A hidden task-owned AppKit fixture covers button type, resize anchoring and close routing; live-window styling remains unobserved until relaunch.

Hover uses an NSTrackingArea on the shared controls view and the AppKit _mouseInGroup: query so real native symbols draw as a group. The native theme widgets receive mouseEnteredOrExited to refresh cached group-hover drawings; manual cell enter/exit forwarding is removed. This is an undocumented AppKit query (also present in Chromium custom titlebar source), isolated to this native presentation bridge; no authentication change.

November11 correction: per-button hidden-fixture captures now prove red/yellow/green all change on hover and exactly restore on exit. Native frame/content bounds agree; UI removes the old transparent outer8px margin and preserves internal layout using inner padding. Live cursor/resize and macOS appearance remain separate.

## Application icon

The GUI process sets its embedded dog icon before starting its host or opening a window. The bridge decodes the image, assigns it to the shared AppKit application and refreshes the Dock tile on the main thread. macOS owns the minimized window thumbnail and its app badge. The bundled app also declares the same icon for launch through its Info.plist. Menu setup is separate from icon assignment.

## November 12 — final reload, copy, startup status and resize UI

The resize bridge adds a transparent six-point rounded ring inside the visible face. Its cursor rows forward actual resizing to AppKit, with original native direction values. Per-instance guards on Fetch concrete-class hooks leave unconfigured instances unchanged. No object_setClass, NSApp/delegate replacement or manual frame-drag arithmetic. Private SPI is guarded; native hidden fixture covers hit tests, controls, close and teardown. Real pointer/drag feel still needs the user test.
