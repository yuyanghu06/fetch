# Install list controls

Goal: a long discovery list must scroll independently without hiding the install
action; provide Select all above it. Keep existing API/import/picker behavior.

Implemented in src/ui/mod.rs: finite Install-page layout, stable bounded scroller,
fixed header and always-present footer/count, variable progress/messages inside
the list, supported-only bulk choice and inactive-state guards. Unsupported rows
remain unavailable; explicit bulk selection can include source-disabled supported
rows without modifying the source flags. Other pages retain previous scrolling.

Independent source verification and Evaluator acceptance complete. Locked native
binary compilation passed (exit0,10.45seconds). No tests or native UI were run by
agents. Completion still needs actual long-list/minimum-window/checkbox/disabled
state/progress checks after relaunch. No importer expansion or provider test was
performed; redacted unsupported-entry reasons are in the development guide.
