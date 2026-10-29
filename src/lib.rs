//! Shared Fetch backend. The developer binary is only one interface to this core.
pub mod api;
pub mod authority;
mod browser;
pub mod credentials;
pub mod development;
pub mod discovery;
pub mod diagnostics;
pub mod storage;
pub mod json;
pub mod oauth;
pub mod identity;
pub mod native_auth;
pub mod installation;
pub mod runner;
pub mod mcp;
pub mod manager;
pub mod application;
pub mod ui;
