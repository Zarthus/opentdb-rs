/// This is the most basic usage of the library.
///
/// It starts a session, it allows you to fully configure
/// the builder, and then sends the request.
///
/// What this does not feature is elegant handling of the Response Code,
/// the following line will throw up if something is wrong:
///
/// ```rs
/// let rs: ApiResponse = opentdb::send_and_parse(rq).unwrap();
/// ```
///
/// This example is also used in the README.
extern crate opentdb;

use opentdb::api_request::ApiRequest;
use opentdb::api_response::ApiResponse;

use opentdb::enums::category::Category;
use opentdb::enums::difficulty::Difficulty;
use opentdb::enums::question_type::QuestionType;
use opentdb::enums::encoding::Encoding;

fn main() {
    // Start a new session so that the service remembers the questions we've received already.
    let token: String = opentdb::session_new()
        .expect("New Session Creation failed")
        .token;

    // Build a new ApiRequest
    let rq: ApiRequest = opentdb::builder()
        .questions(25)
        .category(Category::Any)
        .difficulty(Difficulty::Any)
        .question_type(QuestionType::All)
        .encoding(Encoding::DefaultEncoding)
        .token(token)
        .build();

    // Create a HTTP request from an ApiRequest, send it, and parse the JSON into an ApiResponse.
    let rs: ApiResponse = opentdb::send_and_parse(rq).unwrap();

    println!("Response Code: {:?}", rs.response_code);
    for results in rs.results {
        println!("  [{}] Question: {}", results.category, results.question);
    }
}
