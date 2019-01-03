extern crate opentdb;

use opentdb::api_request::ApiRequest;
use opentdb::api_response::ApiResponse;

fn main() {
    // Our custom Base URL
    let base_url = "http://localhost/otdb_api";

    // Wrong: Uses default URL.
    let token: String = opentdb::session_new(None)
        .expect("New Session Creation failed")
        .token;

    // Right: Uses custom URL.
    let token_custom: String = opentdb::session_new(Some(base_url))
        .expect("New Session Creation failed")
        .token;

    // Right: Specify .base_url() so the request knows the base url.
    let rq: ApiRequest = opentdb::builder()
        .base_url(base_url.to_string())
        .token(token.to_string())
        .build();

    // Right: The ApiRequest struct has knowledge of the base_url
    let _rs: ApiResponse = opentdb::send_and_parse(rq).unwrap();

    // Wrong: We're not passing the base_url Option
    opentdb::session_reset(&token, None).unwrap();

    // Right: Uses custom URL
    opentdb::session_reset(&token_custom, Some(base_url)).unwrap();
 }
