use image::{DynamicImage, GenericImageView, RgbImage};

use crate::color::Color;
use std::sync::Arc;

pub struct Coord {
    pub u: f64,
    pub v: f64,
}

pub trait HasColor {
    fn color(&self, c: &Coord) -> Color;
}

pub struct Texture(Arc<dyn HasColor + Send + Sync>);

impl Texture {
    pub fn new<T: HasColor + Send + Sync + 'static>(value: T) -> Self {
        Texture(Arc::new(value))
    }
}

impl HasColor for Texture {
    fn color(&self, c: &Coord) -> Color {
        self.0.color(c)
    }
}

pub struct Constant {
    pub color: Color,
}

impl HasColor for Constant {
    fn color(&self, _: &Coord) -> Color {
        self.color
    }
}

pub struct UV();

impl HasColor for UV {
    fn color(&self, c: &Coord) -> Color {
        Color {
            r: c.u,
            g: 0.0,
            b: c.v,
        }
    }
}

pub struct Image {
    image: RgbImage,
    width: f64,
    height: f64,
}

impl Image {
    pub fn new(path: &str) -> Self {
        let reader = image::io::Reader::open(path)
            .unwrap_or_else(|err| panic!("failed to open image: {}", err));
        let image = reader
            .decode()
            .unwrap_or_else(|err| panic!("failed to decode image: {}", err));
        let width = image.width() as f64;
        let height = image.height() as f64;
        Image {
            width,
            height,
            image: image.to_rgb8(),
        }
    }
}

impl HasColor for Image {
    fn color(&self, c: &Coord) -> Color {
        let x = (c.u * self.width).trunc() as u32;
        let y = ((1.0 - c.v) * self.height).trunc() as u32;
        let pixel = self.image.get_pixel(x, y);
        Color {
            r: pixel[0] as f64 / 255.0,
            g: pixel[1] as f64 / 255.0,
            b: pixel[2] as f64 / 255.0,
        }
    }
}
