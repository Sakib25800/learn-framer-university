//! This module should contain all tests that test a single webserver route.
//!
//! Each `/api/v1` sub-API should have its own module, with submodules divided by
//! the specific endpoint (e.g. `list`, `create`, `read`, `update`, `delete`).
//!
//! ## Example
//!
//! - testing all the ways authentication works or fails on a specific route
//! - testing error behaviour of a route
//! - testing output serialization of a route
//! - testing query param combinations of a route

pub mod auth;
