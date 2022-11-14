use std::{fs, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the plugins directory and generate the code that can be included to load all plugins
    // The plugins are written to a output file in the target dir and can be included from the lib
    // The output file is generated in the target dir so that it is not checked in to git

    let mut entries = fs::read_dir("../plugins")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    for plugin in entries {
        println!("{}", plugin.display());
    }

    Ok(())
}
