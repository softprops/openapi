//! Support for OpenApi version 3.0.1 specification.
//!
//! See the
//! [specification](https://github.com/OAI/OpenAPI-Specification/blob/0dd79f6/versions/3.0.1.md)
//! for more information.

mod schema;
mod components;

pub use v3_0::schema::*;
pub use v3_0::components::*;
