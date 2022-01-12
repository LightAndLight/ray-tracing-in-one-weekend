use std::fmt::Write;

use clap::Parser;

pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl std::fmt::Display for Dimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.width.fmt(f)?;
        f.write_char(',')?;
        self.height.fmt(f)
    }
}

#[derive(Debug)]
pub enum ParseDimensionsError {
    ParseIntError(std::num::ParseIntError),
    MissingComma,
}

impl From<std::num::ParseIntError> for ParseDimensionsError {
    fn from(err: std::num::ParseIntError) -> Self {
        ParseDimensionsError::ParseIntError(err)
    }
}

impl std::str::FromStr for Dimensions {
    type Err = ParseDimensionsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, suffix) = s
            .split_once(',')
            .ok_or(ParseDimensionsError::MissingComma)?;
        let width = usize::from_str(prefix)?;
        let height = usize::from_str(suffix)?;
        Ok(Dimensions { width, height })
    }
}

impl std::fmt::Display for ParseDimensionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseDimensionsError::ParseIntError(err) => err.fmt(f),
            ParseDimensionsError::MissingComma => {
                f.write_str("dimensions value is missing a comma")
            }
        }
    }
}

impl std::error::Error for ParseDimensionsError {}

#[derive(Parser)]
pub struct Cli {
    /// Image dimensions.
    #[clap(short, long, default_value_t = Dimensions{width: 1920, height: 1080})]
    pub dimensions: Dimensions,

    /// Number of threads to use [default: the number of physical cores present]
    #[clap(short, long)]
    pub num_threads: Option<usize>,

    /// Rays per pixel.
    #[clap(long, default_value_t = 10)]
    pub rays_per_pixel: usize,

    /// Max recursion depth per ray.
    #[clap(long, default_value_t = 50)]
    pub recursion_depth: usize,
}
