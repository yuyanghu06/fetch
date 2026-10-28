# Native human authority

This folder creates temporary authority for Fetch's native Settings window and asks macOS to confirm deliberate app approvals and secret changes. Browser or agent tokens cannot create a Settings window capability. A secret-change capability binds the live window, server and exact mutation fingerprint and expires shortly after confirmation.

The Objective-C bridge uses LocalAuthentication and the traditional login Keychain. Password entry remains in macOS. Keychain operations do not invoke shell utilities or silently fall back to files. Native behavior, local-build identity and prompts have not been tested in this implementation pass.

D79 compilation built this Objective-C bridge and linked the native binary without calling its functions. The compiler emits four classic-Keychain deprecation warnings; those do not establish a runtime failure and do not authorize switching the selected storage mechanism. Native prompts/Keychain behavior remain untested.
