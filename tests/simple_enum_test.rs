extern crate opentdb;

use opentdb::enums::category::Category;
use opentdb::enums::difficulty::Difficulty;
use opentdb::enums::question_type::QuestionType;
use opentdb::enums::encoding::Encoding;
use opentdb::enums::response_code::ResponseCode;

#[test]
fn enum_has_sane_default_test() {
    assert_eq!(Category::Any as u8, Category::default() as u8);
    assert_eq!(Difficulty::Any.value(), Difficulty::default().value());
    assert_eq!(QuestionType::All.value(), QuestionType::default().value());
    assert_eq!(Encoding::DefaultEncoding.value(), Encoding::default().value());
    assert_eq!(ResponseCode::Success as u8, ResponseCode::default() as u8);
}

/// Ensure we map to the correct API expected value
#[test]
fn enum_difficulty_value_literals_test() {
    assert_eq!("0", Difficulty::Any.value());
    assert_eq!("easy", Difficulty::Easy.value());
    assert_eq!("medium", Difficulty::Medium.value());
    assert_eq!("hard", Difficulty::Hard.value());
}

/// Ensure we map to the correct API expected value
#[test]
fn enum_encoding_value_literals_test() {
    assert_eq!("default", Encoding::DefaultEncoding.value());
    assert_eq!("legacy", Encoding::Legacy.value());
    assert_eq!("url3986", Encoding::Url.value());
    assert_eq!("base64", Encoding::Base64.value());
}

/// Ensure we map to the correct API expected value
#[test]
fn enum_question_type_value_literals_test() {
    assert_eq!("all", QuestionType::All.value());
    assert_eq!("multiple", QuestionType::Multiple.value());
    assert_eq!("boolean", QuestionType::Boolean.value());
}
