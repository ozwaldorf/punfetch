// Required to be in scope to derive `Render`
use owo_colors::OwoColorize;

// Recommended to use `default-features=false` to avoid binary deps
use punfetch::{
    // For the info module, add `features=["sysinfo"]`
    info::{sys, ColorInfo, HostInfo},
    Distro, Printer, Render,
};

#[derive(Render)]
struct ExampleInfo {
    pub field: String,
    pub field_two: String,
    pub optional: Option<String>,
}

impl ExampleInfo {
    pub fn new() -> Self {
        Self {
            field: "value".into(),
            field_two: "this is another value".into(),
            optional: None,
        }
    }
}

fn main() {
    let sys = sys();
    let host_info = HostInfo::new(&sys);

    // Create a printer with true colors enabled
    let mut renderer = Printer::new(Some(true));

    // Add some ascii art
    renderer.with_distro(Distro::search(host_info.distro.clone()));

    // Add the host info
    renderer.with_info(host_info);

    // Add our custom info
    renderer.with_info(ExampleInfo::new());

    // Add a color bar
    renderer.with_info(ColorInfo);

    // Render the output!
    renderer.render()
}
