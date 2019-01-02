#[derive(Deserialize, Debug)]
/// The Question Type is what describes what sort of question type this is.
///
/// The "Multiple" question type will give multiple-choice questions, where there
/// is one correct answer and multiple incorrect ones.
///
/// The "Boolean" question type will give you true/false questions.
pub enum QuestionType {
    /// Do not filter question types
    All,
    /// Filter by Multiple Choice questions
    Multiple,
    /// Filter by true/false questions
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
