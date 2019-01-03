#[cfg(test)]

extern crate opentdb;

use opentdb::builder::simple::ApiBuilderSimple;

use opentdb::enums::category::Category;
use opentdb::enums::difficulty::Difficulty;
use opentdb::enums::question_type::QuestionType;
use opentdb::enums::encoding::Encoding;

#[test]
fn simple_builder_test() {
    let mut sb: ApiBuilderSimple = opentdb::simple_builder();

    let build = sb.questions(3)
        .base_url(String::from("localhost"))
        .category(Category::Animals as u8)
        .difficulty(String::from(Difficulty::Any.value()))
        .question_type(String::from(QuestionType::Multiple.value()))
        .encoding(String::from(Encoding::Url.value()))
        .token(String::from("abc"))
        .build();

    assert_eq!("url3986", build.encoding);
    assert_eq!("multiple", build.question_type);
    assert_eq!("0", build.difficulty);
    assert_eq!(3, build.questions);
}

/// The Simple Builder is not limited to what is within Enums, so the builder SHOULD be able
/// to deal with any arbitrary data. If the user uses something unsupported, breakage MIGHT occur,
/// but at the same time should something unsupported and new or shiny happen, the user can use it.
#[test]
fn simple_builder_accepts_nonconforming_values() {
    let mut sb: ApiBuilderSimple = opentdb::simple_builder();

    let build = sb.questions(3)
        .category(100)
        .difficulty(String::from("some difficulty"))
        .question_type(String::from("unique question type"))
        .encoding(String::from("different encoding"))
        .token(String::from("some token"))
        .build();

    assert_eq!("different encoding", build.encoding);
    assert_eq!("unique question type", build.question_type);
    assert_eq!("some difficulty", build.difficulty);
    assert_eq!(100, build.category);
    assert_eq!(3, build.questions);
    assert_eq!("some token", build.token);
}
