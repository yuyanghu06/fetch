# Projects

`mod.rs` saves explicitly registered project names and canonical folder paths in
Fetch's private `projects/` records. It also supplies shared/project server
membership checks. It never changes client configuration or project contents.

The native API owns registration permission. Coding-client authority is derived
from an approved connection resource; a model-supplied path is not authority.
Workspace browsing is separate from these saved connection bindings.
