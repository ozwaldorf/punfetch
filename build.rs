use serde_json::json;
use serde_yaml::from_reader;
use std::{
    error::Error,
    fs::{read_to_string, write},
    path::Path,
};
use tera::{Context, Tera};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=distros.yaml");
    println!("cargo:rerun-if-changed=src/distros/distro.tera");

    let out_dir = std::env::var("OUT_DIR")?;
    let mut tera = Tera::default();
    let mut data: serde_json::Value = from_reader(read_to_string("distros.yaml")?.as_bytes())?;

    // for each distro
    for (name, obj) in data.as_object_mut().unwrap().iter_mut() {
        let obj = obj.as_object_mut().unwrap();

        // add rust type name
        let reg = regex::Regex::new(r"[\s_\-./!@]").unwrap();
        let type_name = reg.replace_all(name.as_str(), "").to_ascii_uppercase();
        obj.insert("type_name".to_string(), json!(type_name));

        // get stripped ascii and width
        let ascii = obj.get("ascii").unwrap().as_str().unwrap();
        let reg = regex::Regex::new(r"\{[0-9]+\}").unwrap();
        let ascii_stripped = reg.replace_all(ascii, "");
        let width = ascii_stripped
            .lines()
            .map(|l| l.chars().count())
            .max()
            .unwrap();
        obj.insert("ascii_stripped".to_string(), json!(ascii_stripped));
        obj.insert("width".to_string(), json!(width));

        // add regex pattern, strip spaces, special chars
        if obj.get("regex").is_none() {
            let reg = regex::Regex::new(r"([\s_\-./!@])").unwrap();
            let regex = reg
                .replace_all(name.as_str(), r"")
                .to_string()
                .to_ascii_lowercase()
                .replace("linux", "");
            obj.insert("regex".to_string(), json!(regex));
        }
    }

    let rust_code = tera.render_str(
        &read_to_string("src/distros/distro.tera")?,
        &Context::from_value(json!({ "distros": data }))?,
    )?;

    write(Path::new(&out_dir).join("distros.rs"), rust_code)?;

    Ok(())
}
