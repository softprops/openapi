#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use std::fs;
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

pub fn from_path<P>(path: P) -> errors::Result<Spec>
where
    P: AsRef<Path>,
{
    Ok(serde_yaml::from_reader::<_, Spec>(fs::File::open(path)?)?)
}

pub fn to_yaml(spec: &Spec) -> errors::Result<String> {
    Ok(serde_yaml::to_string(spec)?)
}

pub fn to_json(spec: &Spec) -> errors::Result<String> {
    Ok(serde_json::to_string_pretty(spec)?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
