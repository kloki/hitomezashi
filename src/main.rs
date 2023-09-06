use clap::Parser;
use image::{
    ImageBuffer,
    ImageError,
    Rgb,
};

const WHITE: [u8; 3] = [255, 255, 255];

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
}

struct Pattern {
    x: Vec<bool>,
    y: Vec<bool>,
    margin: u32,
}

impl Pattern {
    fn new(x: Vec<bool>, y: Vec<bool>) -> Self {
        Pattern { x, y, margin: 5 }
    }

    fn image_size(&self) -> (u32, u32) {
        (
            (self.x.len() - 1) as u32 * self.margin + 1,
            (self.y.len() - 1) as u32 * self.margin + 1,
        )
    }

    fn on_stitch(&self, x: u32, y: u32) -> bool {
        match (
            x % self.margin,
            y % self.margin,
            (x / self.margin) as usize,
            (y / self.margin) as usize,
        ) {
            (0, 0, _, _) => true,
            (0, _, x_index, y_index) => (y_index % 2 == 0) == self.x[x_index],
            (_, 0, x_index, y_index) => (x_index % 2 == 0) == self.y[y_index],
            _ => false,
        }
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
        if !pattern.on_stitch(x, y) {
            *pixel = Rgb(WHITE);
        }
    }

    imgbuf.save(args.output_file)?;
    Ok(())
}
