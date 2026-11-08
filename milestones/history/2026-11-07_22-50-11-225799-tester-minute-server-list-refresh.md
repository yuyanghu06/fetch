# Check automatic server-list refresh

- Local timestamp / acting role / milestone: 2026-11-07T22:50:11.225799-05:00 / independent Tester, recorded by root / periodic-server-refresh
- Requested scope: Focused native-UI state checks and locked build for60second app refresh.
- Starting repository state: Coder implementation complete; Tester exclusively owns src/ui/auth_tests.rs.
- Work performed / changed files: Added two tests (+39lines) using existing disposable desktop_fixture. All Iced Tasks are dropped, so no request/window task executes. Root inspected test diff/logs and copies evidence into milestone folder.
- Decisions: No new dependency, timer test infrastructure or60second wait. Tests cover event scheduling/reply handling; source inspection establishes the exact60s subscription and unchanged GET path.
- Checks and evidence: cargo test --locked --lib ui::auth_tests::automatic_refresh_ -- --nocapture passes2/2,170filtered,0.01s. Covers automatic collapsed-list update on Connect, pending/clientNone guards and unchanged1s expanded-console Tick. cargo build --locked --bin fetch passes12.17s with12existing warnings. Scoped git diff --check clean.
- Resulting state / acceptance: Independent Tester PASS. No HTTP, native app/window, service/MCP, account or Keychain action; no live timing claim.
- Remaining work and handoff: Final independent source/evidence review; deliberate native relaunch/observation later.
