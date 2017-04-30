#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use std::fs;
use std::io::Read;
use std::path::Path;

mod schema;
pub use schema::Spec;

pub mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Yaml(::serde_yaml::Error);
            Serialize(::serde_json::Error);
        }
    }
}

/// deserialize an open api spec from a path
pub fn from_path<P>(path: P) -> errors::Result<Spec>
where
    P: AsRef<Path>,
{
    from_reader(fs::File::open(path)?)
}

/// deserialize an open api spec from type which implements Read
pub fn from_reader<R>(read: R) -> errors::Result<Spec>
where
    R: Read,
{
    Ok(serde_yaml::from_reader::<R, Spec>(read)?)
}

/// serialize to a yaml string
pub fn to_yaml(spec: &Spec) -> errors::Result<String> {
    Ok(serde_yaml::to_string(spec)?)
}

/// serialize to a json string
pub fn to_json(spec: &Spec) -> errors::Result<String> {
    Ok(serde_json::to_string_pretty(spec)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Just tests if the deserialization does not blow up. But does not test correctness
    #[test]
    fn can_deserialize() {
        for entry in fs::read_dir("data/").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            // cargo test -- --nocapture to see this message
            println!("Testing if {:?} is deserializable", path);
            from_path(path).unwrap();
        }
    }
}
