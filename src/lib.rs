//! Openapi provides structures and support for serializing and deserializing [openapi](https://github.com/OAI/OpenAPI-Specification) specifications
//!
//! # Examples
//!
//! Typical use deserialing an existing to a persisted spec to rust form or
//! visa versa
//!
//! The hyper client should be configured with tls.
//!
//! ```no_run
//! extern crate openapi;
//!
//! fn main() {
//!   match openapi::from_path("path/to/openapi.yaml") {
//!     Ok(spec) => println!("spec: {:?}", spec),
//!     Err(err) => println!("error: {}", err)
//!   }
//! }
//! ```
//!
//! # Errors
//!
//! Operations typically result in a `openapi::Result` Type which is an alias
//! for Rust's
//! built-in Result with the Err Type fixed to the
//! [openapi::errors::Error](errors/struct.Error.html) enum type. These are
//! provided
//! using [error_chain](https://github.com/brson/error-chain) crate so their
//! shape and behavior should be consistent and familiar to existing
//! error_chain users.
//!
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate url;
extern crate url_serde;
extern crate semver;

use std::fs;
use std::io::Read;
use std::path::Path;

pub mod v2;
pub mod v3_0;

const MINIMUM_OPENAPI30_VERSION: &str = ">= 3.0";


/// errors that openapi functions may return
pub mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Yaml(::serde_yaml::Error);
            Serialize(::serde_json::Error);
            SemVerError(::semver::SemVerError);
        }

        errors {
            UnsupportedSpecFileVersion(version: ::semver::Version) {
                description("Unsupported spec file version")
                display("Unsupported spec file version ({}). Expected {}", version, ::MINIMUM_OPENAPI30_VERSION)
            }
        }
    }
}
pub use errors::{Result, ResultExt};

/// Supported versions of the OpenApi.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum OpenApi {
    /// Version 2.0 of the OpenApi specification.
    ///
    /// Refer to the official
    /// [specification](https://github.com/OAI/OpenAPI-Specification/blob/0dd79f6/versions/2.0.md)
    /// for more information.
    V2(v2::Spec),

    /// Version 3.0.1 of the OpenApi specification.
    ///
    /// Refer to the official
    /// [specification](https://github.com/OAI/OpenAPI-Specification/blob/0dd79f6/versions/3.0.1.md)
    /// for more information.
    #[allow(non_camel_case_types)]
    V3_0(v3_0::Spec),
}

/// deserialize an open api spec from a path
pub fn from_path<P>(path: P) -> errors::Result<OpenApi>
where
    P: AsRef<Path>,
{
    from_reader(fs::File::open(path)?)
}

/// deserialize an open api spec from type which implements Read
pub fn from_reader<R>(read: R) -> errors::Result<OpenApi>
where
    R: Read,
{
    Ok(serde_yaml::from_reader::<R, OpenApi>(read)?)
}

/// serialize to a yaml string
pub fn to_yaml(spec: &OpenApi) -> errors::Result<String> {
    Ok(serde_yaml::to_string(spec)?)
}

/// serialize to a json string
pub fn to_json(spec: &OpenApi) -> errors::Result<String> {
    Ok(serde_json::to_string_pretty(spec)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Just tests if the deserialization does not blow up. But does not test correctness
    #[test]
    fn can_deserialize() {
        for entry in fs::read_dir("data/v2").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            // cargo test -- --nocapture to see this message
            println!("Testing if {:?} is deserializable", path);
            from_path(path).unwrap();
        }
    }
}
