extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate percent_encoding;

extern crate reqwest;

use percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};

pub mod builder;
pub mod api_request;
pub mod api_response;
pub mod enums;

use crate::builder::simple::ApiBuilderSimple;
use crate::builder::strict::ApiBuilderStrict;
use crate::api_request::ApiRequest;
use crate::api_response::{ApiResponse, ApiSessionNew, ApiSessionReset};

/// Creates a new API Builder (strict by default)
pub fn builder() -> ApiBuilderStrict {
    strict_builder()
}

/// Creates a new API Builder (permissive: permits your custom values)
pub fn simple_builder() -> ApiBuilderSimple {
    ApiBuilderSimple::new()
}

/// Creates a new API Builder (strict: requires use of library enums)
pub fn strict_builder() -> ApiBuilderStrict {
    ApiBuilderStrict::new()
}

/// Send a blocking Request and return the reqwest Response
///
/// All data provided by their API is available under the
/// [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/)
/// as is written [on the Open Trivia Database website](https://opentdb.com/api_config.php).
pub fn send(request: ApiRequest) -> Result<reqwest::Response, reqwest::Error> {
    reqwest::Client::new()
        .get(to_url(request).as_str())
        .send()
}

/// Send a blocking Request and return a parsed Api Response.
///
/// All data provided by their API is available under the
/// [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/)
/// as is written [on the Open Trivia Database website](https://opentdb.com/api_config.php).
pub fn send_and_parse(request: ApiRequest) -> Result<ApiResponse, reqwest::Error> {
    let response: ApiResponse = send(request)
        .expect("Failed to send Request")
        .json()?;

    Ok(response)
}

/// Converts an ApiRequest to a GET url for requests to `api.php`.
///
/// Example full URL: https://opentdb.com/api.php?amount=10&category=11&difficulty=easy&type=multiple&encode=base64
fn to_url(request: ApiRequest) -> String {
    let mut url: String = request.base_url + "/api.php";

    url.push_str("?amount=");
    url.push_str(&request.questions.to_string());
    url.push_str("&category=");
    url.push_str(&request.category.to_string());
    url.push_str("&difficulty=");
    url.push_str(&request.difficulty);
    url.push_str("&type=");
    url.push_str(&request.question_type);
    url.push_str("&encoding=");
    url.push_str(&request.encoding);

    if !request.token.is_empty() {
        url.push_str("&token=");
        url.push_str(&request.token);
    }

    percent_encode(url.as_bytes(), DEFAULT_ENCODE_SET).to_string()
}

/// Retrieve a new API token
///
/// The Open Trivia Database provides a completely free JSON API for use in programming projects.
/// Use of this API does not require a API Key.
///
/// Session Tokens are unique keys that will help keep track of the questions the API has already retrieved.
/// By appending a Session Token to a API Call, the API will never give you the same question twice.
///
/// Once all questions are exhausted, you will have to reset the session or request a new token.
///
/// Session Tokens will be deleted after 6 hours of inactivity.
///
/// Calls: https://opentdb.com/api_token.php?command=request
///
/// The token will look like this: `955ccb0cfee1435e15c1d82cf5ed9528b1ed6fed28353043e8f91f4dcda3cff1`
pub fn session_new(base_url: Option<&str>) -> Result<ApiSessionNew, reqwest::Error> {
    let rq: ApiSessionNew = reqwest::Client::new()
        .get((base_url.unwrap_or("https://opentdb.com").to_string() + "/api_token.php?command=request").as_str())
        .send()
        .expect("Failed to send Request")
        .json()?;

    Ok(rq)
}

/// Reset your session token
///
/// Over the lifespan of a Session Token, there will eventually reach a point where you have exhausted all the possible questions in the database.
/// At this point, the API will respond with the appropriate "Response Code" (Code 4: Token Empty).
/// From here, you can either "Reset" the Token, which will wipe all past memory, or you can ask for a new one.
///
/// Session Tokens will be deleted after 6 hours of inactivity.
///
/// Calls: https://opentdb.com/api_token.php?command=reset&token=YOURTOKENHERE
///
/// footnote: The API returns a blank page when querying an empty or wrong API key.
pub fn session_reset(token: &str, base_url: Option<&str>) -> Result<ApiSessionReset, reqwest::Error> {
    let mut url: String = base_url.unwrap_or("https://opentdb.com").to_string() + "/api_token.php?command=reset&token=";
    url.push_str(percent_encode(token.as_bytes(), DEFAULT_ENCODE_SET).to_string().as_str());

    let rq: ApiSessionReset = reqwest::Client::new()
        .get(url.as_str())
        .send()
        .expect("Failed to send Request")
        .json()?;

    Ok(rq)
}
