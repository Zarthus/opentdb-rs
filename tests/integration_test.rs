extern crate opentdb;
extern crate reqwest;

use opentdb::api_request::{ApiRequest};
use opentdb::api_response::{ApiResponse};

use opentdb::enums::category::Category;
use opentdb::enums::difficulty::Difficulty;
use opentdb::enums::question_type::QuestionType;
use opentdb::enums::encoding::Encoding;

/// This is a full end-to-end test, touching 90% of the library
/// It negates some of the enum choices, we don't want to test the remote API.
/// In short: This gives us a 99% confidence level what we changed is OK.
///
/// In total it sends and parses three HTTP requests.
///
/// TODO: Mock Requests from the server.
#[test]
fn integration_end_to_end_test() {
    let token: String = opentdb::session_new()
        .expect("Session Creation failed")
        .token;

    assert!(!token.is_empty());

    let rq: ApiRequest = opentdb::builder()
        .questions(3)
        .category(Category::Any)
        .difficulty(Difficulty::Any)
        .question_type(QuestionType::Boolean)
        .encoding(Encoding::DefaultEncoding)
        .token(token.to_string())
        .build();

    let rs: ApiResponse = opentdb::send_and_parse(rq)
        .expect("Sending HTTP request and Parsing it failed");

    assert_eq!(opentdb::enums::response_code::ResponseCode::Success as u8, rs.response_code);
    assert_eq!(3, rs.results.len());

    for results in rs.results {
        assert_eq!("boolean", results.question_type);
    }

    let reset_token: String = opentdb::session_reset(&token)
        .expect("Session Reset failed")
        .token;

    assert!(!reset_token.is_empty());

    assert_eq!(token, reset_token);
}
