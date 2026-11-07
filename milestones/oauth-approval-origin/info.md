# Browser approval origin

Goal: Codex can submit Fetch's approval page without Fetch rejecting its own form with403.

Scope: Preserve browser origin for Fetch's own approval form without disclosing referrers to other origins. Keep exact origin, browser cookie, CSRF, native confirmation and PKCE checks. Also remove the requested Connect apps cleanup checkbox/prompt and unused UI state.

Acceptance: Focused registration/approval/token test through the complete router, null/foreign-origin refusal, native build and direct source review. Real browser/Codex/native Mac approval remain distinct observations.

Status: Scoped repair and cleanup implemented. New flow fails on original policy and passes after fix;38focused checks, signed native build16.15s, strict verification and direct source review pass. All follow-up work performed directly, without independent subagent checks. Real browser/Codex/Mac approval retry remains unobserved; explicit Quit/signed relaunch needed.


Live follow-up: Chrome console confirms self-only form-action blocks the callback after Mac approval. Scoped callback-origin policy implemented; focused regression fails before and passes after. Signed rebuild and actual fetch_codex/browser retry underway. Computer control denies Warp/Terminal and cannot target the bare development executable; any required app Quit/Mac password remains a user handoff.


Final connection follow-up: actual saved-login fetch_codex now initializes and loads405Fetch tools after the separate initial-version negotiation correction. This establishes saved Fetch authorization reuse and client catalog availability; it does not exercise every upstream tool/provider. See ../mcp-version-negotiation/info.md.
