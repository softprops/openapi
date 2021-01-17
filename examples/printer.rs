use anyhow::Result;

fn main() -> Result<()> {
    if let Some(path) = std::env::args().nth(1) {
        let spec = openapi::from_path(path)?;
        /*for (path, op) in spec.paths {
            println!("{}", path);
            println!("{:#?}", op);
        }
        for (name, definition) in spec.definitions {
            println!("{}", name);
            println!("{:#?}", definition);
        }*/
        println!("{}", openapi::to_json(&spec)?);
    }
    Ok(())
}
