#[derive(Clone, Debug)]
pub enum Section {
    Stitch,
    A,
    B,
}

impl Section {
    fn flip(&self) -> Self {
        match self {
            Section::A => Section::B,
            Section::B => Section::A,
            Section::Stitch => Section::Stitch,
        }
    }
}

pub struct Pattern {
    x: Vec<bool>,
    y: Vec<bool>,
    column_width: u32,
    color_map: Vec<Vec<Section>>,
}

impl Pattern {
    pub fn new(x: Vec<bool>, y: Vec<bool>, column_width: u32) -> Self {
        let mut color_map = vec![vec![Section::A; y.len()]; x.len()];
        let mut current_color = Section::A;
        for y_cor in 0..y.len() {
            if (x[0] && x[0] == y[y_cor]) || !x[0] && x[0] != y[y_cor] {
                current_color = current_color.flip();
            }
            color_map[0][y_cor] = current_color.clone();
        }
        for y_cor in 0..y.len() {
            current_color = color_map[0][y_cor].clone();
            for x_cor in 1..x.len() {
                if y_cor % 2 == 0 && x[x_cor] {
                    current_color = current_color.flip();
                }
                if y_cor % 2 != 0 && !x[x_cor] {
                    current_color = current_color.flip();
                }
                color_map[x_cor][y_cor] = current_color.clone();
            }
        }

        Pattern {
            x,
            y,
            column_width,
            color_map,
        }
    }

    pub fn image_size(&self) -> (u32, u32) {
        (
            (self.x.len() - 1) as u32 * self.column_width + 1,
            (self.y.len() - 1) as u32 * self.column_width + 1,
        )
    }

    pub fn get_section(&self, x: u32, y: u32) -> Section {
        match (
            x % self.column_width,
            y % self.column_width,
            (x / self.column_width) as usize,
            (y / self.column_width) as usize,
        ) {
            (0, 0, _, _) => Section::Stitch,
            (0, _, x_index, y_index) if (y_index % 2 == 0) == self.x[x_index] => Section::Stitch,
            (_, 0, x_index, y_index) if (x_index % 2 == 0) == self.y[y_index] => Section::Stitch,
            (_, _, x_index, y_index) => self.color_map[x_index][y_index].clone(),
        }
    }
}
