#[derive(Deserialize, Debug)]
pub enum Difficulty {
    Any,
    Easy,
    Medium,
    Hard,
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::Any
    }
}

impl Difficulty {
    pub fn value(&self) -> &str {
        match *self {
            Difficulty::Any => { "0" },
            Difficulty::Easy => { "easy" },
            Difficulty::Medium => { "medium" },
            Difficulty::Hard => { "hard" },
        }
    }
}
