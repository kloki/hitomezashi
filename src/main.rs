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
mod pattern;

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
    /// Stitch color
    #[arg(long, value_parser=clap::value_parser!(Color), default_value="0,0,0")]
    color_stitch: Color,
    /// Color of section A
    #[arg(long, value_parser=clap::value_parser!(Color), default_value="255,255,255")]
    color_a: Color,
    /// Color of section B
    #[arg(long, value_parser=clap::value_parser!(Color), default_value="0,255,255")]
    color_b: Color,
}

#[derive(Debug, Clone)]
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

impl Color {
    fn into_array(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

fn seed(input: String) -> Vec<bool> {
    //magically work with even//odd and consonant/vowel
    let vowels = ['a', 'e', 'i', 'o', 'u', '0', '2', '4', '6', '8', '0'];
    input.chars().map(|x| vowels.contains(&x)).collect()
}

fn mulitply_vec<T: Clone>(input: Vec<T>, i: usize) -> Vec<T> {
    let mut output = input.clone();
    for _ in 0..i {
        output.append(&mut input.clone())
    }
    output
}

fn main() -> Result<(), ImageError> {
    let args = Args::parse();
    let x_seed = mulitply_vec(seed(args.x), args.multiply);
    let y_seed = mulitply_vec(seed(args.y), args.multiply);
    let pattern = Pattern::new(x_seed, y_seed);
    let (width, height) = pattern.image_size();
    let mut imgbuf = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = match pattern.get_section(x, y) {
            Section::Stitch => Rgb(args.color_stitch.into_array()),
            Section::A => Rgb(args.color_a.into_array()),
            Section::B => Rgb(args.color_b.into_array()),
        }
    }

    imgbuf.save(args.output_file)?;
    Ok(())
}
