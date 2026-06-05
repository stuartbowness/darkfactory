//! `darkfactory-render` library root.
//!
//! The crate deterministically renders `contract.md` from authored acceptance
//! fields. The `core` module tree is the pure, IO-free library; the `cli` module
//! is the thin transport (file IO, exit codes, stderr) that the `main.rs` binary
//! calls. Keeping `cli` in the library lets the binary be driven in-process by
//! unit tests as well as end-to-end via `std::process::Command`.
//!
//! A local module named `core` is permitted in Rust 2021 because the standard
//! `core` crate is reached via `::core`. Reference this tree as
//! `crate::core::types`.
pub mod cli;
pub mod core {
    pub mod coverage;
    pub mod parse;
    pub mod render;
    pub mod shadow;
    pub mod types;
}
