# Workspace bulk clipboard

`bulk.rs` reads definitions with the existing Workspace file API. It writes either a single card or a bundle to the clipboard, including relative filenames and the source text. Toolbar Paste accepts both formats, uses create-only Save and reports per-file refusals. Card Paste replaces the selected file with the first entry before trying to create any remaining files. Both paste actions keep the original bytes for the same app and call the existing conversion API when changing apps. The parent Workspace view owns the toolbar, selection and result display.

Each copied entry carries the displayed card name separately from the relative filename. Cross-app paste passes it to the converter if the source definition needs a name.
