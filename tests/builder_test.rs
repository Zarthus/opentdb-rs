extern crate opentdb;

use opentdb::builder::simple::ApiBuilderSimple;

use opentdb::enums::category::Category;
use opentdb::enums::difficulty::Difficulty;
use opentdb::enums::question_type::QuestionType;
use opentdb::enums::encoding::Encoding;

/// This is a built-in limitation by the remote API.
///
/// - A Maximum of 50 Questions can be retrieved per call.
#[test]
fn builder_simple_panics_at_plus50_questions_test() {
    let result = std::panic::catch_unwind(|| {
        let mut sb  = opentdb::simple_builder();

        let build = sb.questions(51)
            .build();
    });
    assert!(result.is_err());
}

/// This is a built-in limitation by the remote API.
///
/// - A Maximum of 50 Questions can be retrieved per call.
#[test]
fn builder_strict_panics_at_plus50_questions_test() {
    let result = std::panic::catch_unwind(|| {
        let mut sb = opentdb::strict_builder();

        let build = sb.questions(51)
            .build();
    });
    assert!(result.is_err());
}
