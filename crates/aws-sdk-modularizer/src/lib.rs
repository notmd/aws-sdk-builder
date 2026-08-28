//! A small, manifest-driven codemod for modular AWS SDK service crates.
//!
//! The downloaded Smithy-RS output is the source of truth. This crate only
//! discovers operation ownership from the Smithy model and adds Cargo/Rust
//! feature gates to that source tree.

pub mod conformance;
pub mod diff;
pub mod manifest;
pub mod model;
pub mod names;
pub mod transform;

pub use manifest::{Manifest, ServiceManifest};
pub use model::{Model, Operation};
pub use transform::{Coverage, TransformOutput};
