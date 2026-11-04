November 4: saved installation records may include private relative program_directories. The record validator rejects invalid folder entries; public projections continue removing all _installation metadata. Startup resolves and checks containment through the installer’s program_paths module.

# Private Fetch server records

ServerRepository reads/writes owner-only local configuration records with checked
directory descriptors, no-follow reads and atomic file replacement. It preserves
internal installation/launch identity while returning a safe status projection.
Credentials and internal installation details are excluded from that projection.

Legacy credential-name/status arrays can still be read as old non-secret metadata;
new installation records do not need them. No legacy credential bytes are opened,
rewritten or deleted by this module. Explicit internal native_setup records carry
an exact executable, arguments, working folder and actual state directory; they are
never inferred from package names or invented login subcommands.

Removing a server now unlinks only its Fetch config.json registration after the
manager has closed its idle owned connection. Native state, old encrypted files,
Keychain items, package files, symlink targets and other servers stay untouched.
The former recursive package/home deletion methods have been removed from the
product path. Removing a registration is not deleting an account or logging out.

Tests use disposable files/symlinks and check state preservation and refusal to
follow a substituted server directory. Full manager/API checks are separate.
