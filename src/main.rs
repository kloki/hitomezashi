use std::str::FromStr;

use clap::Parser;
use image::{
    ImageBuffer,
    ImageError,
    Rgb,
};
use pattern::{
    Pattern,
    Section,
};
use seed::Seed;
mod pattern;
mod seed;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Seed string for x axis
    x: String,
    /// Seed string for y axis
    y: String,
    /// Name of png file
    #[arg(short, long, default_value = "./output.png")]
    output_file: String,
    /// multiply input seeds
    #[arg(short, long, default_value_t = 0)]
    multiply: usize,
    /// pixel with of one column
    #[arg(short, long, default_value_t = 5)]
    column_width: u32,
    /// Stitch color
    #[arg(long, value_parser=clap::value_parser!(Color), default_value="0,0,0")]
    color_stitch: Color,
    /// Color of section A
    #[arg(long, value_parser=clap::value_parser!(Color), default_value="255,255,255")]
    color_a: Color,
    /// Color of section B
    #[arg(long, value_parser=clap::value_parser!(Color), default_value="0,255,255")]
    color_b: Color,
    /// Append mirror of seed to itself, will make the image symmetric
    #[arg(long, default_value_t = false)]
    mirror: bool,
}

#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}
impl FromStr for Color {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(",").collect();
        if split.len() != 3 {
            return Err("Not an rgb color: r,b,g".to_string());
        }
        Ok(Color {
            r: split[0].parse::<u8>().map_err(|e| format!("{e}"))?,
            g: split[1].parse::<u8>().map_err(|e| format!("{e}"))?,
            b: split[2].parse::<u8>().map_err(|e| format!("{e}"))?,
        })
    }
}

impl From<Color> for [u8; 3] {
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b]
    }
}

fn main() -> Result<(), ImageError> {
    let args = Args::parse();
    let mut x_seed = Seed::magic_seed(args.x).multiply(args.multiply);
    let mut y_seed = Seed::magic_seed(args.y).multiply(args.multiply);
    if args.mirror {
        x_seed = x_seed.mirror();
        y_seed = y_seed.mirror();
    }
    let pattern = Pattern::new(x_seed.seed, y_seed.seed, args.column_width);

    let (width, height) = pattern.image_size();
    let mut imgbuf = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = match pattern.get_section(x, y) {
            Section::Stitch => Rgb(args.color_stitch.into()),
            Section::A => Rgb(args.color_a.into()),
            Section::B => Rgb(args.color_b.into()),
        }
    }

    imgbuf.save(args.output_file)?;
    Ok(())
}
