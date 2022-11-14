use quote::{format_ident, quote};
use std::{
    env,
    fs::{self, write},
    io,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = fs::read_to_string("./Cargo.toml")?;
    let mut doc = manifest.parse::<toml_edit::Document>()?;

    let entries = fs::read_dir("../../plugins")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    for plugin in entries.clone() {
        println!("cargo:rebuilt-if-changed={}", plugin.display());
        doc["dependencies"][plugin.file_name().unwrap().to_str().unwrap()]["path"] =
            toml_edit::value(format!("{}", plugin.display()));
    }
    fs::write("./Cargo.toml", doc.to_string())?;
    let entries_str = entries
        .into_iter()
        .map(|e| {
            format_ident!(
                "{}",
                e.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
                    .replace("-", "_")
            )
        })
        .collect::<Vec<_>>();

    let result = quote! {
        use plugin_lib::PluginTrait;
        pub fn load_plugins() -> Vec<Box<dyn PluginTrait>> {
            vec![
                #(
                    Box::new(#entries_str::Plugin::new()),
                )*
            ]
        }
    };
    write(
        format!("{}{}", env::var("OUT_DIR").unwrap(), "/plugin_shim.rs"),
        result.to_string(),
    )?;
    Ok(())
}
