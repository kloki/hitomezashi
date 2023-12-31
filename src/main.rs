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
