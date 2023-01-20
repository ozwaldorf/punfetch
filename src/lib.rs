//! # punfetch (lib)
//!
//! A blazingly fast system info fetcher library written in Rust.
//!
//! For the binary, see [punfetch (bin)](https://crates.io/crates/punfetch).
//!
//! ## Custom fetch example
//!
//! ```rust,no_run
#![doc = include_str ! ("../examples/custom_fetch.rs")]
//! ```

use std::{default::Default, fmt::Write};

use image::DynamicImage;
use onefetch_ascii::AsciiArt;
use onefetch_image::get_best_backend;
use owo_colors::{AnsiColors, DynColors};

pub use punfetch_derive::Render;

pub use crate::distros::Distro;

mod distros;

/// A collection of structs implementing [`Render`]
pub mod info;

/// Trait for types be added to the [`Printer`].
pub trait Render {
    fn render(&self, color: DynColors) -> Vec<String>;
}

impl Render for Vec<String> {
    fn render(&self, _: DynColors) -> Vec<String> {
        self.to_owned()
    }
}

impl Render for String {
    fn render(&self, _: DynColors) -> Vec<String> {
        vec![self.to_owned()]
    }
}

/// Generic system fetch printer accepting a distro or image, some configuration, and [`Render`]-able info
pub struct Printer<'a> {
    pub color: DynColors,
    pub info: Vec<Box<dyn Render + 'a>>,
    pub ascii: Option<AsciiArt<'a>>,
    pub image: Option<DynamicImage>,
}

impl<'a> Default for Printer<'a> {
    fn default() -> Self {
        Self {
            color: DynColors::Ansi(AnsiColors::Default),
            info: Vec::new(),
            ascii: None,
            image: None,
        }
    }
}

impl<'a> Printer<'a> {
    /// Provide an image to the renderer
    #[inline]
    pub fn with_image(&mut self, image: DynamicImage) {
        self.image = Some(image);
    }

    /// Provide a distro to the renderer
    #[inline]
    pub fn with_ascii(&mut self, ascii: AsciiArt<'a>) {
        self.ascii = Some(ascii);
    }

    /// Provide a color to the renderer for text
    #[inline]
    pub fn with_color(&mut self, color: DynColors) {
        self.color = color;
    }

    /// Render an object and add it to the text lines
    #[inline]
    pub fn with_info<R: Render + 'a>(&mut self, info: R) {
        self.info.push(Box::new(info));
    }

    /// Render the ascii art and print it to stdout
    #[inline]
    pub fn render(&mut self) {
        let mut buf = String::new();
        let color = self.color;
        let lines = self.info.iter().flat_map(|i| i.render(color)).collect();

        if let Some((image, backend)) = self
            .image
            .as_ref()
            .and_then(|img| get_best_backend().map(|backend| (img, backend)))
        {
            buf.push_str(backend.add_image(lines, image, 32).unwrap().as_str());
        } else if let Some(ref mut art) = &mut self.ascii {
            let padding = art.width();
            let mut lines = lines.iter();
            loop {
                match (art.next(), lines.next()) {
                    (Some(art), Some(line)) => {
                        writeln!(buf, "  {art}  {line}")
                    }
                    (Some(art), None) => writeln!(buf, "  {art}"),
                    (None, Some(line)) => writeln!(buf, "  {}  {line}", " ".repeat(padding)),
                    (None, None) => {
                        write!(buf, "\n").unwrap();
                        break;
                    },
                }
                .expect("failed to write to buffer");
            }
        } else {
            for line in lines.iter() {
                writeln!(buf, "{line}").expect("failed to write to buffer");
            }
            buf.push('\n');
        }

        print!("{buf}")
    }
}
