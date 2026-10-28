# Configuration file chooser

Goal: choose an existing config file from the Install MCPs path row using the
standard macOS picker, while retaining manual input and explicit discovery.

Source implemented with the existing AppKit bridge and a Rust async wrapper.
Single-file selection includes hidden folders and fills an absolute path;
Cancel preserves the current path. Duplicate requests/late window results are
guarded and close/Quit cancels the picker. No config read/import happens merely
from selection. No API or dependency was added.

Independent source verification/Evaluator acceptance complete. Locked native
binary compilation passed (exit0,11.35seconds). No app/dialog or tests ran;
interactive selection/cancel/hidden-file/close-reopen/Quit acceptance remains
pending. Follow current snapshot and to-do; Root owns journal writes.
