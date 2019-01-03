use crate::enums::category::Category;
use crate::enums::difficulty::Difficulty;
use crate::enums::question_type::QuestionType;
use crate::enums::encoding::Encoding;
use crate::api_request::ApiRequest;

/// The Strict Builder is limited to what is within Enums, so the builder only deals
/// with data that is KNOWN to the library. If the user uses something unsupported, the builder
/// will not accept it. Should, at some point, something unsupported be needed, the
/// ApiBuilderSimple will permit this
#[allow(dead_code)]
#[derive(Default)]
pub struct ApiBuilderStrict {
    base_url: String,
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
            base_url: String::from("https://opentdb.com/"),
            questions: 10 as u8,
            category: Category::Any,
            difficulty: Difficulty::Any,
            question_type: QuestionType::All,
            encoding: Encoding::DefaultEncoding,
            token: String::from(""),
        }
    }

    pub fn base_url(&mut self, base_url: String) -> &mut Self {
        self.base_url = base_url;
        self
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
            base_url: self.base_url.to_string(),
            questions: self.questions,
            category: self.category as u8,
            difficulty: self.difficulty.value().to_string(),
            question_type: self.question_type.value().to_string(),
            encoding: self.encoding.value().to_string(),
            token: self.token.clone(),
        }
    }
}
