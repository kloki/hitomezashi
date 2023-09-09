const MAGIC: [char; 14] = [
    'a', 'e', 'i', 'o', 'u', ' ', ',', '.', '0', '2', '4', '6', '8', '0',
];
#[derive(Clone)]
pub struct Seed {
    pub seed: Vec<bool>,
}

impl Seed {
    pub fn magic_seed(input: String) -> Self {
        let seed: Vec<_> = input.chars().map(|x| MAGIC.contains(&x)).collect();
        Seed { seed }
    }

    pub fn multiply(&mut self, n: usize) -> Self {
        let mut new_seed = self.seed.clone();
        for _ in 0..n {
            new_seed.append(&mut self.seed.clone());
        }
        Seed { seed: new_seed }
    }

    pub fn mirror(&self) -> Self {
        let mut new_seed = self.seed.clone();
        new_seed.append(&mut self.seed.iter().rev().map(|x| *x).collect::<Vec<_>>());
        Seed { seed: new_seed }
    }
}
