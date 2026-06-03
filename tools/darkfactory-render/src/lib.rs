//! `darkfactory-render` library root.
//!
//! The crate deterministically renders `contract.md` from authored acceptance
//! fields. This module tree is the pure `core` library; the CLI binary
//! (`main.rs`) is a thin wrapper added in a later task.
//!
//! A local module named `core` is permitted in Rust 2021 because the standard
//! `core` crate is reached via `::core`. Reference this tree as
//! `crate::core::types`.
pub mod core {
    pub mod parse;
    pub mod types;
}
