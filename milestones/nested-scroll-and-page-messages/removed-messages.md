# Messages removed from beneath navigation

The native page used to insert `self.error` and `self.notice` directly after its
navigation. Both rendering branches are removed on Servers, Install and Connect.
No backend response, stored error or contextual Console/dialog message is erased.

Fixed notice text that no longer appears in that page-level area:

- “An installation is already running. Monitoring that existing operation.”
- “Server registration removed. Its files and credentials were preserved.”
- “Manual key changes saved in macOS Keychain. Account access has not been verified.”
- “Complete the provider's browser sign-in, then Fetch will show the result.”
- “Saved. Review server state for any required next action.”
- “Server restart failed. Review its state before retrying.”
- “Server folder open request completed.”

Error messages removed from that area include the reported “The server is
restarting or its waiting queue is full.” That text is returned by the backend;
any other returned error could also occupy the same slot. A finite text inventory
therefore cannot enumerate every upstream/network/backend message.

Current fixed local errors that also lose that page-level display:

- “Choose an assigned key entry and enter its value.”
- “Fetch did not confirm storing the manual key.”
- “Enter 1–86400 seconds and 1–64 concurrent calls.”
- “Fetch did not confirm opening the server folder.”
- Deletion failures append “Refreshing server status before another delete attempt;
  deletion will not be retried automatically.” to the returned error, or use
  “The server did not confirm deletion.” when there is no confirmed result.

Other affected errors include native-client setup, file/folder chooser, discovery,
installation/progress, tool-catalog reads, server-status reads, manual-key save,
configuration, restart/status and opening-folder request failures. Actual returned
text varies. These operations still execute; their errors are retained internally.

“Server restart completed.” had already been removed in the preceding refresh
animation pass, so it is not counted as a new removal here. Console/server inline
errors, installation-dialog errors, rejected definitions and progress details stay
in their existing locations. No replacement message surface was added.
