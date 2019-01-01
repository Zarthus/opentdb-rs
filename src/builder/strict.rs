use crate::enums::category::Category;
use crate::enums::difficulty::Difficulty;
use crate::enums::question_type::QuestionType;
use crate::enums::encoding::Encoding;
use crate::api_request::ApiRequest;

#[allow(dead_code)]
#[derive(Default)]
pub struct ApiBuilderStrict {
    questions: u8,
    category: Category,
    difficulty: Difficulty,
    question_type: QuestionType,
    encoding: Encoding,
    token: String,
}

impl ApiBuilderStrict {
    pub fn new() -> Self {
        ApiBuilderStrict {
            questions: 10 as u8,
            category: Category::Any,
            difficulty: Difficulty::Any,
            question_type: QuestionType::All,
            encoding: Encoding::DefaultEncoding,
            token: String::from(""),
        }
    }

    pub fn questions(&mut self, questions: u8) -> &mut Self {
        if questions > 50 {
            panic!("Maximum of 50 questions are allowed.");
        }

        self.questions = questions;
        self
    }

    pub fn category(&mut self, category: Category) -> &mut Self {
        self.category = category;
        self
    }

    pub fn difficulty(&mut self, difficulty: Difficulty) -> &mut Self {
        self.difficulty = difficulty;
        self
    }

    pub fn question_type(&mut self, question_type: QuestionType) -> &mut Self {
        self.question_type = question_type;
        self
    }

    pub fn encoding(&mut self, encoding: Encoding) -> &mut Self {
        self.encoding = encoding;
        self
    }

    pub fn token(&mut self, token: String) -> &mut Self {
        self.token = token;
        self
    }

    pub fn build(&self) -> ApiRequest {
        ApiRequest {
            questions: self.questions,
            category: self.category as u8,
            difficulty: self.difficulty.value().to_string(),
            question_type: self.question_type.value().to_string(),
            encoding: self.encoding.value().to_string(),
            token: self.token.clone(),
        }
    }
}
