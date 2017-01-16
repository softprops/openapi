extern crate openapi;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let spec = openapi::from_path(path);
        for (path, _) in spec.paths {
            println!("{}", path)
        }
    }

}
