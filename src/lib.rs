extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use std::fs;
use std::path::Path;

mod types;
pub use types::*;

pub fn from_path<P>(path: P) -> Spec
    where P: AsRef<Path>
{
    serde_yaml::from_reader::<fs::File, Spec>(fs::File::open(path).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
