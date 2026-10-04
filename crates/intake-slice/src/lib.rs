#![forbid(unsafe_code)]

//! The vertical slice: open a case on the picked protocol through the governor, run Loom over it
//! and execute local actions until blocked (stories S3-S5). Temporary: Commission's local runtime
//! loop replaces it.
//!
//! [`case`] opens the slice's case through `beyond10x/governor` and reports the workspace's new
//! `HEAD` to it; the slice never evaluates Canon itself. The loop and the local executor are not
//! built yet.

pub mod case;
