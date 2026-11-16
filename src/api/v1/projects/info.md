# Registered project folders

`mod.rs` wraps ProjectRegistry with native-only GET/POST operations. It returns
saved IDs, names, canonical folders and exact project MCP addresses. Registration
never scans unrelated folders or edits project/client files. Ordinary bearer
tokens cannot list or register projects through these routes.
