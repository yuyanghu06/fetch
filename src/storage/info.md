# Saved server records

The repository reads and atomically saves normalized, non-secret local server records. It hashes opaque IDs only for directory names, so IDs cannot escape their server directory. Private directory/file permissions, directory handles and no-follow opens protect owned paths. Invalid records fail the read rather than disappearing.

Disk state never proves a running process or a valid provider login. Reads start with stopped runtime and unknown authentication. This first increment stores local records only; remote records and execution-profile persistence will be added with their implementations. No HTTP write route uses this primitive yet. Callers must classify launch arguments as non-secret before saving; the repository cannot infer whether arbitrary arguments contain credentials.

The injected root is a trusted application/developer path; ancestor path selection belongs to startup. Owned root, server directories and files reject symlink leaves, wrong ownership and broad permissions. New directories are private from creation, rather than being tightened afterward. Opens below the root use held directory handles; missing roots and unreadable/broken symlink roots are distinguished. The current root opener does not reject symlinks in caller-selected ancestor directories.
