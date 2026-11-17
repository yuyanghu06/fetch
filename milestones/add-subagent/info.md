# Add subagent

On Workspace’s Subagents tab, the blue Add subagent button with white text sits just before the Claude and Codex logos. It opens a blank, focused definition editor in the existing popup. Paste is immediately left of Copy. Use Paste or ⌘V, enter a filename and check the shown destination before clicking Save. Definitions copied from Fetch’s Subagents list can use the existing app conversion.

Save creates a file in the selected app’s agent folder. If the name is taken, Fetch keeps the original file and the draft, then shows the error and filename. Edit still saves existing definitions through the same private API. [Save contract](../../docs/architecture/workspace.md#add-subagent-save)

Seven focused checks passed across widgets and the disposable HTTP fixture. Independent review accepted the feature. The signed development build and strict verification passed. Testing used an offscreen app and fixture clipboard.

- [Blue toolbar button](toolbar-blue.png)
- [Blank popup with adjacent Paste and Copy](blank-popup.png)
- [Save at minimum window size](minimum-save.png)
- [Filename conflict at minimum window size](duplicate-filename.png)
- [Focused results](focused-tests.txt)

Quit Fetch and reopen with `./scripts/run-development.sh` to try it in the live window.
