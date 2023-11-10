use clap::Parser;
use hitomezashi::{
    pattern::{Pattern, Section},
    seed::Seed,
};
use image::{ImageBuffer, ImageError, Rgb};
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Your eth adress
    eth: String,
    /// Name of png file
    #[arg(short, long, default_value = "./output.png")]
    output_file: String,
}

fn random_color(rng: &mut Pcg64) -> [u8; 3] {
    [
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
    ]
}

fn main() -> Result<(), ImageError> {
    let args = Args::parse();
    let y_seed = Seed::magic_seed(args.eth.clone()).mirror();
    let x_seed = Seed::magic_seed(args.eth.clone()).mirror();
    let pattern = Pattern::new(x_seed.seed, y_seed.seed, 10);

    let (width, height) = pattern.image_size();
    let mut imgbuf = ImageBuffer::new(width, height);
    let mut rng: Pcg64 = Seeder::from(args.eth).make_rng();

    let color_a = random_color(&mut rng);
    let color_b = random_color(&mut rng);
    let color_stitch = random_color(&mut rng);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = match pattern.get_section(x, y) {
            Section::Stitch => Rgb(color_stitch),
            Section::A => Rgb(color_a),
            Section::B => Rgb(color_b),
        }
    }

    imgbuf.save(args.output_file)?;
    Ok(())
}
