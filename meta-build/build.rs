use cargo_toml::Manifest;
use serde::Serialize;
use std::{
    env, fs, path::Path,
};
use syn::Item;

#[derive(Debug, Default, Serialize)]
struct Properties {
    name: String,
    attributes: Vec<String>,
}

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let manifest_dir = Path::new(&manifest_dir);

    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

    let manifest = Manifest::from_path(manifest_dir.join("Cargo.toml"))?;

    if let Some(lib) = manifest.lib {
        if let Some(lib_path) = lib.path {
            let props = parse_lib(&manifest_dir.join(lib_path))?;

            fs::write(
                &out_dir.join("schema.json"),
                serde_json::to_string_pretty(&props)?,
            )?;

            fs::write(
                &out_dir.join("main.rs"),
                r#"""
                fn main() -> {
                    println!("{}", include_str!("schema.json"));
                }
                """#,
            )?;
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}

fn parse_lib(lib_path: &Path) -> Result<Vec<Properties>, Error> {
    let data = fs::read_to_string(&lib_path)?;

    let input = syn::parse_file(&data)?;

    Ok(input.items.into_iter().filter_map(|item| {
        match item {
            Item::Struct(item) => {
                Some(Properties {
                    name: format!("{}", item.ident),
                    // TODO
                    attributes: Default::default(),
                })
            },
            _ => None,
        }
    }).collect())
}
