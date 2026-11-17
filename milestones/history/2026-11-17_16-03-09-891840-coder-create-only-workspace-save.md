# Implement the create-only Workspace save

- Local timestamp / acting role / milestone: 2026-11-17T16:03:09-05:00 / Coder / Add subagent
- Requested scope: Let Add create a new subagent without replacing an occupied name. Preserve regular file edits.
- Starting repository state: The private PUT route accepted file contents and could create or replace recognized files. The user selected refusing a duplicate and keeping the draft.
- Work performed / changed files: workspace/mod.rs in the API reads the conditional header after authorization. The Workspace service validates the destination and publishes a completed new file with a same-directory hard link, which fails if another file already has that name. NativeClient sends the header for Add.
- Decisions: Return 412 WORKSPACE_FILE_ALREADY_EXISTS on an occupied filename. Use the existing save reply on success. Continue the existing Edit path for requests without a condition.
- Checks and evidence: Reviewed the diff and ran git diff --check. Disposable HTTP verification is pending.
- Resulting state / acceptance: Backend and native client source are ready for the UI and tests.
- Remaining work and handoff: Test a protected existing file, creation at a different filename and a normal edit. Confirm the duplicate message leaves the popup usable.
