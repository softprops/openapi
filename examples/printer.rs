extern crate openapi;

use std::io::Write;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        match openapi::from_path(path) {
            Ok(spec) => {
                /*for (path, op) in spec.paths {
                    println!("{}", path);
                    println!("{:#?}", op);
                }
                for (name, definition) in spec.definitions {
                    println!("{}", name);
                    println!("{:#?}", definition);
                }*/
                println!("{}", openapi::to_json(&spec).unwrap());
            }
            Err(e) => {

                let stderr = &mut ::std::io::stderr();
                let errmsg = "Error writing to stderr";

                writeln!(stderr, "error: {}", e).expect(errmsg);

                for e in e.iter().skip(1) {
                    writeln!(stderr, "caused by: {}", e).expect(errmsg);
                }

                // The backtrace is not always generated. Try to run this example
                // with `RUST_BACKTRACE=1`.
                if let Some(backtrace) = e.backtrace() {
                    writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
                }

                ::std::process::exit(1);
            }
        }
    }
}
