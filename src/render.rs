//! Render ascii or image art, and text info

use std::fmt::Write;

use image::DynamicImage;
use onefetch_image::get_best_backend;
use owo_colors::{AnsiColors, DynColors};

pub use punfetch_derive::Render;

use crate::distros::Distro;

/// Implemented by all structs that can be added to the [`Printer`].
pub trait Render {
    fn render(&self, color: DynColors) -> Vec<String>;
}

/// Fetch printer accepting some art, render-able info, and configuration
pub struct Printer<'a> {
    pub color: Option<DynColors>,
    pub color_mode: Option<bool>,
    pub info: Vec<Box<dyn Render + 'a>>,
    pub distro: Option<Distro>,
    pub image: Option<DynamicImage>,
}

impl<'a> Printer<'a> {
    pub fn new(color_mode: Option<bool>) -> Self {
        Self {
            color_mode,
            color: None,
            info: Vec::new(),
            distro: None,
            image: None,
        }
    }

    /// Provide an image to the renderer
    #[inline]
    pub fn with_image(&mut self, image: DynamicImage) {
        self.image = Some(image);
    }

    /// Provide a distro to the renderer
    #[inline]
    pub fn with_distro(&mut self, distro: Distro) {
        self.distro = Some(distro);
    }

    /// Provide a color to the renderer for text
    #[inline]
    pub fn with_color(&mut self, color: DynColors) {
        self.color = Some(color);
    }

    /// Render an object and add it to the text lines
    #[inline]
    pub fn with_info<S: Render + 'a>(&mut self, info: S) {
        self.info.push(Box::new(info));
    }

    /// Get renderer's current main color
    fn main_color(&self) -> DynColors {
        self.color.unwrap_or_else(|| {
            self.distro
                .map(|d| d.color())
                .unwrap_or_else(|| DynColors::Ansi(AnsiColors::Default))
        })
    }

    /// Render the ascii art and print it to stdout
    #[inline]
    pub fn render(&mut self) {
        let mut buf = String::new();
        let color = self.main_color();
        let lines = self
            .info
            .iter()
            .map(|i| i.render(color))
            .flatten()
            .collect();

        if let Some((image, backend)) = self
            .image
            .as_ref()
            .and_then(|img| get_best_backend().map(|backend| (img, backend)))
        {
            buf.push_str(backend.add_image(lines, image, 32).unwrap().as_str());
        } else if let Some(distro) = &self.distro {
            let mut art = distro.ascii(self.color_mode);
            let width = art.width();
            let mut lines = lines.iter();

            loop {
                match (art.next(), lines.next()) {
                    (Some(art), Some(line)) => {
                        writeln!(buf, "  {art}  {line}")
                    }
                    (Some(art), None) => writeln!(buf, "  {art}"),
                    (None, Some(line)) => writeln!(buf, "  {}  {line}", " ".repeat(width)),
                    (None, None) => break,
                }
                .expect("failed to write to buffer");
            }
        } else {
            for line in lines.iter() {
                writeln!(buf, "{line}").expect("failed to write to buffer");
            }
        }

        print!("{buf}")
    }
}
