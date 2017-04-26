#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde;
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
        }
    }
}

pub fn from_path<P>(path: P) -> errors::Result<Spec>
where
    P: AsRef<Path>,
{
    Ok(serde_yaml::from_reader::<_, Spec>(fs::File::open(path)?)?)
}

pub fn to_yaml(spec: &Spec) -> String {
    serde_yaml::to_string(spec).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
