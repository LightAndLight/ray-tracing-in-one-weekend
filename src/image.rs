use crate::color::Color;
use std::io::{self, Write};

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}

impl Image {
    pub fn render<W: Write>(&self, buffer: &mut W) -> io::Result<()> {
        writeln!(buffer, "P3")?;
        writeln!(buffer, "{} {}", self.width, self.height)?;
        writeln!(buffer, "255")?;
        (0..self.height).try_for_each(|y| {
            let offset = self.width * y;
            (0..self.width).try_for_each(|x| {
                self.data[offset + x].render_ppm(buffer);
                Ok(())
            })
        })
    }
}
