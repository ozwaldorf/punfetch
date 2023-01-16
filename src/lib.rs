//! # punfetch (lib)
//!
//! A blazingly fast system info fetcher library written in Rust.
//!
//! For the binary, see [punfetch (bin)](https://crates.io/crates/punfetch).
//!
//! ## Custom fetch example
//!
//! ```rust,no_run
#![doc = include_str ! ("../examples/custom_fetch.rs")]
//! ```

pub use crate::{distros::Distro, render::*};

mod distros;
mod render;

#[cfg(feature = "sysinfo")]
pub mod info;
