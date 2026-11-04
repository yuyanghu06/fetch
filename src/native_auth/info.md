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
