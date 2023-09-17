use clap::Parser;
use hitomezashi::{
    color::Color,
    pattern::{
        Pattern,
        Section,
    },
    seed::Seed,
};
use image::{
    ImageBuffer,
    ImageError,
    Rgb,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Your -anme
    name: String,
    /// Name of png file
    #[arg(short, long, default_value = "./output.png")]
    /// Stitch color
    output_file: String,
    #[arg(long, value_parser=clap::value_parser!(Color), default_value="0,0,0")]
    color_stitch: Color,
    /// Color of section A
    #[arg(long, value_parser=clap::value_parser!(Color), default_value="255,255,255")]
    color_a: Color,
    /// Color of section B
    #[arg(long, value_parser=clap::value_parser!(Color), default_value="0,255,255")]
    color_b: Color,
}

fn main() -> Result<(), ImageError> {
    let args = Args::parse();
    let x_seed = Seed::magic_seed(args.name).mirror();
    let y_seed = x_seed.clone();
    let pattern = Pattern::new(x_seed.seed, y_seed.seed, 10);

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
