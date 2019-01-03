extern crate opentdb;
extern crate reqwest;
#[cfg(test)]
extern crate mockito;

use opentdb::api_request::{ApiRequest};
use opentdb::api_response::{ApiResponse};

use opentdb::enums::category::Category;
use opentdb::enums::difficulty::Difficulty;
use opentdb::enums::question_type::QuestionType;
use opentdb::enums::encoding::Encoding;

use mockito::{mock, Matcher};

fn mock_response<P: Into<Matcher>>(route: P, file: &str, status_code: usize) -> mockito::Mock {
    mock("GET", route)
      .with_status(status_code)
      .with_body_from_file(format!("tests/data/response/{}", file).as_str())
}

/// This is a full end-to-end test, touching 90% of the library
/// It negates some of the enum choices, we don't want to test the remote API.
/// In short: This gives us a 99% confidence level what we changed is OK.
///
/// In total it sends and parses three HTTP requests.
///
/// TODO: Mock Requests from the server.
#[test]
fn integration_end_to_end_test() {
    let _mock_session_new = mock_response("/api_token.php?command=request", "api_session_new.json", 200).create();
    let _mock_api_response = mock_response(Matcher::Regex(r"^/api.php".to_string()), "api_3q_typeboolean.json", 200).create();
    let _mock_session_reset = mock_response(
        "/api_token.php?command=reset&token=8112fe454d59bce3a24c6b00953cd14c8e9154c64d7f68da456ef2d35503ff8b",
        "api_session_reset.json",
        200
    ).create();

    let token: String = opentdb::session_new(Some(&mockito::server_url()))
        .expect("Session Creation failed")
        .token;

    assert!(!token.is_empty());

    let rq: ApiRequest = opentdb::builder()
        .questions(3)
        .base_url(mockito::server_url())
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

    let reset_token: String = opentdb::session_reset(&token, Some(&mockito::server_url()))
        .expect("Session Reset failed")
        .token;

    assert!(!reset_token.is_empty());

    assert_eq!(token, reset_token);

    _mock_session_new.assert();
    _mock_api_response.assert();
    _mock_session_reset.assert();
}

#[test]
fn integration_fail_response_code_1_test() {
    let _mock_api_response_fail1 = mock_response(Matcher::Any, "api_fail_respcode_1.json", 200).create();

    let rq: ApiRequest = opentdb::builder().base_url(mockito::server_url()).build();
    let rs: ApiResponse = opentdb::send_and_parse(rq).unwrap();

    _mock_api_response_fail1.assert();
    assert_eq!(1, rs.response_code as u8);
}

#[test]
fn integration_fail_response_code_2_test() {
    let _mock_api_response_fail2 = mock_response(Matcher::Any, "api_fail_respcode_2.json", 200).create();

    let rq: ApiRequest = opentdb::builder().base_url(mockito::server_url()).build();
    let rs: ApiResponse = opentdb::send_and_parse(rq).unwrap();

    _mock_api_response_fail2.assert();
    assert_eq!(2, rs.response_code as u8);
}

#[test]
fn integration_fail_response_code_3_test() {
    let _mock_api_response_fail3 = mock_response(Matcher::Any, "api_fail_respcode_3.json", 200).create();

    let rq: ApiRequest = opentdb::builder().base_url(mockito::server_url()).build();
    let rs: ApiResponse = opentdb::send_and_parse(rq).unwrap();

    _mock_api_response_fail3.assert();
    assert_eq!(3, rs.response_code as u8);
}

#[test]
fn integration_fail_response_code_4_test() {
    let _mock_api_response_fail4 = mock_response(Matcher::Any, "api_fail_respcode_4.json", 200).create();

    let rq: ApiRequest = opentdb::builder().base_url(mockito::server_url()).build();
    let rs: ApiResponse = opentdb::send_and_parse(rq).unwrap();

    _mock_api_response_fail4.assert();
    assert_eq!(4, rs.response_code as u8);
}
