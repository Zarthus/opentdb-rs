/// An API response from `/api.php`.

/// A struct representing the response from the server for the main API.
///
/// Example Response
/// ```json
/// {
///   "response_code": 0,
///   "results": [
///     {
///       ...
///     },
///     ...
///   ]
/// }
/// ```
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub response_code: u8,
    pub results: Vec<ApiResult>
}

/// A struct representing a collection of Results of a `ApiResponse`.
///
/// Example Result:
/// ```json
/// {
///   "category": "Entertainment: Music",
///   "type": "multiple",
///   "difficulty": "easy",
///   "question": "The Red Hot Chili Pepper song &quot;Give It Away&quot; is from what album?",
///   "correct_answer": "Blood Sugar Sex Magik",
///   "incorrect_answers": [
///      "One Hot Minute",
///      "By the Way",
///      "Stadium Arcadium"
///   ]
/// }
/// ```
/// If a type = "boolean", incorrect_answers will only fill a single slot.
#[derive(Deserialize, Debug)]
pub struct ApiResult {
    pub category: String,
    #[serde(rename = "type")]
    pub question_type: String,
    pub difficulty: String,
    pub question: String,
    pub correct_answer: String,
    pub incorrect_answers: Vec<String>
}

/// A struct representing the response of a new Session being created.
///
/// Example Result:
/// ```json
/// {
///   "response_code": 0,
///   "response_message": "Token Generated Successfully!",
///   "token": "2ef34d9dd5a8fd73312fc9ab4cc5a3a1ef392bf751d9ef74216fd55dbd1e6025"
/// }
/// ```
#[derive(Deserialize, Debug)]
pub struct ApiSessionNew {
    pub response_code: u8,
    pub response_message: String,
    pub token: String
}

/// A struct representing the response of a Session being reset.
///
/// Example Result:
/// ```json
/// {
///   "response_code": 0,
///   "token": "2ef34d9dd5a8fd73312fc9ab4cc5a3a1ef392bf751d9ef74216fd55dbd1e6025"
/// }
/// ```
#[derive(Deserialize, Debug)]
pub struct ApiSessionReset {
    pub response_code: u8,
    pub token: String
}
