// Required to be in scope to derive `Render`
use owo_colors::{
    AnsiColors::{Green, Red, Yellow},
    DynColors, OwoColorize,
};

// Recommended to use `default-features=false` to avoid binary deps
use punfetch::info::{PercentBar, DEFAULT_BAR_WIDTH};
use punfetch::{
    // For the info module, add `features=["sysinfo"]`
    info::{sys, ColorBar, HostInfo},
    Distro,
    Printer,
    Render,
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
    let mut renderer = Printer::default();

    // Find the distro
    let distro = Distro::search(host_info.distro.clone());

    // Add some ascii art with true colors
    renderer.with_ascii(distro.ascii(Some(true)));

    // Add a color
    renderer.with_color(distro.color(Some(true)));

    // Add the host info
    renderer.with_info(host_info);

    // Let's add a percentage bar
    renderer.with_info(PercentBar {
        title: "Example Stats".to_string(),
        total: 100.0,
        items: vec![
            // title, value, percentage
            ("foo".to_string(), 50.0, 1.0),
            ("bar".to_string(), 25.0, 0.25),
            ("baz".to_string(), 13.0, 0.13),
        ],
        colors: vec![
            DynColors::Ansi(Green),
            DynColors::Ansi(Yellow),
            DynColors::Ansi(Red),
        ],
        width: DEFAULT_BAR_WIDTH,
    });

    // Add our custom info
    renderer.with_info(ExampleInfo::new());

    // Add a color bar
    renderer.with_info(ColorBar::default());

    // Render the output!
    renderer.render()
}
