use crate::enums::category::Category;
use crate::enums::difficulty::Difficulty;
use crate::enums::question_type::QuestionType;
use crate::enums::encoding::Encoding;
use crate::api_request::ApiRequest;

/// The Simple Builder is not limited to what is within Enums, so the builder SHOULD be able
/// to deal with any arbitrary data. If the user uses something unsupported, breakage MIGHT occur,
/// but at the same time should something unsupported and new or shiny happen, the user can use it.
#[allow(dead_code)]
#[derive(Default)]
pub struct ApiBuilderSimple {
    questions: u8,
    category: u8,
    difficulty: String,
    question_type: String,
    encoding: String,
    token: String,
}

impl ApiBuilderSimple {
    pub fn new() -> Self {
        ApiBuilderSimple {
            questions: 10,
            category: Category::Any as u8,
            difficulty: Difficulty::Any.value().to_string(),
            question_type: QuestionType::All.value().to_string(),
            encoding: Encoding::DefaultEncoding.value().to_string(),
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

    pub fn category(&mut self, category: u8) -> &mut Self {
        self.category = category;
        self
    }

    pub fn difficulty(&mut self, difficulty: String) -> &mut Self {
        self.difficulty = difficulty;
        self
    }

    pub fn question_type(&mut self, question_type: String) -> &mut Self {
        self.question_type = question_type;
        self
    }

    pub fn encoding(&mut self, encoding: String) -> &mut Self {
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
            category: self.category,
            difficulty: self.difficulty.to_owned(),
            question_type: self.question_type.to_owned(),
            encoding: self.encoding.to_owned(),
            token: self.token.clone(),
        }
    }
}
