#[derive(Deserialize, Debug)]
/// Which difficulty we want the Questions to be in.
pub enum Difficulty {
    /// Do not filter by difficulty
    Any,
    /// Filter questions by "easy" difficulty
    Easy,
    /// Filter questions by "medium" difficulty
    Medium,
    /// Filter questions by "hard" difficulty
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
