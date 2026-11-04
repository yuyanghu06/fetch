# Saved server records

Deletion cleans only validated Fetch-owned package/private-state paths relative to held descriptors. Recursive unlink never follows symlinks and tolerates normal nested package modes. Recursive removal batches disk flushing at the selected subtree boundary: descendants are unlinked without individual flushes, then the held containing directory is flushed once, including already-missing retry targets. Credential/config file flushes stay separate. config.json is removed last, with restoration attempted if final directory removal fails. Trees beyond128 levels or the existing4096 entries per-directory limit fail explicitly; no secure-erasure claim.

The repository reads and atomically saves normalized, non-secret local server records. It hashes opaque IDs only for directory names, so IDs cannot escape their server directory. Private directory/file permissions, directory handles and no-follow opens protect owned paths. Invalid records fail the read rather than disappearing.

Disk state never proves a running process or a valid provider login. Public repository reads start with stopped runtime and unknown authentication. Backend-only `get_saved`/`list_saved` retain import associations for reuse. Records support local/remote connections, execution settings, Fetch-owned observations and reference-only credential bindings; the manager overlays actual process and authentication state. Internal source identity is removed from the public projection. Callers must classify launch arguments as non-secret before saving; the repository cannot infer whether arbitrary arguments contain credentials.

The injected root is a trusted application/developer path; ancestor path selection belongs to startup. Owned root, server directories and files reject symlink leaves, wrong ownership and broad permissions. New directories are private from creation, rather than being tightened afterward. Opens below the root use held directory handles; missing roots and unreadable/broken symlink roots are distinguished. The current root opener does not reject symlinks in caller-selected ancestor directories.

Remote records use the same recognized public endpoint-path profile as discovery/capture. Ordinary saved JSON must not contain unclassified source URL paths that could embed credentials. Custom layouts outside that profile are unsupported at this source checkpoint; no encrypted path-URL feature is implied.


Installation metadata accepts either the existing source_path association or a
validated direct_identity digest, with shared source_name/installation_id/
protected_args fields. Metadata stays backend-only and is stripped from public
server reads. Existing records require no migration; identity survives edits to
the saved launch settings.

Deletion preservation fixtures are in deletion_tests.rs. They verify nested cleanup,
symlink-target/sibling/runtime preservation, missing-package retry and private-state
cleanup that leaves config.json intact. No native credential store is used.
