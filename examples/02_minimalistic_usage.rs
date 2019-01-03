/// This is the smallest possible usage of the library.
///
/// It features no sessions, no resetting of the session, and just
/// using the default builder values.
extern crate opentdb;

use opentdb::api_request::ApiRequest;
use opentdb::api_response::ApiResponse;

fn main() {
    // Build a new ApiRequest
    let rq: ApiRequest = opentdb::builder().build();

    // Create a HTTP request from an ApiRequest, send it, and parse the JSON into an ApiResponse.
    let rs: ApiResponse = opentdb::send_and_parse(rq).unwrap();

    println!("Response Code: {:?}", rs.response_code);
    for results in rs.results {
        println!("  [{}] Question: {}", results.category, results.question);
    }
}
