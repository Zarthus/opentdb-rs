#[derive(Deserialize, Debug)]
pub enum QuestionType {
    All,
    Multiple,
    Boolean
}

impl Default for QuestionType {
    fn default() -> Self {
        QuestionType::All
    }
}

impl QuestionType {
    pub fn value(&self) -> &str {
        match *self {
            QuestionType::All => { "all" },
            QuestionType::Multiple => { "multiple" },
            QuestionType::Boolean => { "boolean" },
        }
    }
}
