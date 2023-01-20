use std::{
    collections::HashMap,
    error::Error,
    fs::{read_to_string, write},
    path::Path,
};

use serde_json::{json, to_value};
use serde_yaml::from_reader;
use tera::{try_get_value, Context, Tera};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=distros.yaml");
    println!("cargo:rerun-if-changed=src/distros/distros.tera");

    let out_dir = std::env::var("OUT_DIR")?;
    let mut tera = Tera::default();
    tera.register_filter("hex_to_rgb", hex_to_rgb_filter);
    tera.register_filter("pascal_case", pascal_case);

    let mut data: serde_json::Value = from_reader(read_to_string("distros.yaml")?.as_bytes())?;

    // for each distro
    for (name, obj) in data.as_object_mut().unwrap().iter_mut() {
        let obj = obj.as_object_mut().unwrap();

        // add rust type name
        let reg = regex::Regex::new(r"[\s_\-./!@]").unwrap();
        let type_name = reg.replace_all(name.as_str(), "").to_ascii_uppercase();
        obj.insert("type_name".to_string(), json!(type_name));

        // Set default color if missing
        let _ = obj.entry("colors").or_insert(json!({
            "ansi": ["default"],
        }));

        // Stripped & width
        let template = obj.get("ascii").unwrap().as_str().unwrap().to_string();
        let reg = regex::Regex::new(r"\{[0-9]+\}").unwrap();
        let stripped_ascii = reg.replace_all(&template, "");
        let stripped_width = stripped_ascii
            .lines()
            .map(|l| l.chars().count())
            .max()
            .unwrap();
        obj.insert("stripped_ascii".to_string(), json!(stripped_ascii));
        obj.insert("stripped_width".to_string(), json!(stripped_width));

        // add regex pattern (not provided: strip spaces + special chars from name)
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
        &read_to_string("src/distros/distros.tera")?,
        &Context::from_value(json!({ "distros": data }))?,
    )?;

    write(Path::new(&out_dir).join("distros.rs"), rust_code)?;

    Ok(())
}

pub fn pascal_case(
    value: &tera::Value,
    _: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let s = try_get_value!("owo_color", "value", String, value);
    let sections: Vec<_> = s.split('_').collect();
    let mut buf = String::new();
    for str in sections {
        let mut chars = str.chars();
        if let Some(f) = chars.next() {
            buf.push_str(&(f.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()))
        }
    }
    Ok(to_value(&buf).unwrap())
}

fn hex_to_rgb_filter(
    value: &tera::Value,
    _args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let hex_string = try_get_value!("hex_to_rgb", "value", String, value);
    let hex_string = match hex_string.strip_prefix('#') {
        Some(s) => s,
        None => return Err(tera::Error::msg("expected hex string starting with `#`")),
    };
    if hex_string.len() != 6 {
        return Err(tera::Error::msg("expected a 6 digit hex string"));
    }
    let channel_bytes = match u32::from_str_radix(hex_string, 16) {
        Ok(n) => n,
        Err(_) => return Err(tera::Error::msg("expected a valid hex string")),
    };
    let r = (channel_bytes >> 16) & 0xFF;
    let g = (channel_bytes >> 8) & 0xFF;
    let b = channel_bytes & 0xFF;

    Ok(json!({
        "r": r,
        "g": g,
        "b": b,
    }))
}
