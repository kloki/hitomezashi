use std::str::FromStr;
#[derive(Debug, Clone, Copy)]
pub struct Color {
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
