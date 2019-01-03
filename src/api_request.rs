/// A struct representing the request to the server for the main API.
///
/// Example: https://opentdb.com/api.php?amount=10&category=10&difficulty=medium&type=multiple&encode=base64
#[derive(Deserialize, Debug)]
pub struct ApiRequest {
    pub base_url: String,
    pub questions: u8,
    pub category: u8,
    pub difficulty: String,
    pub question_type: String,
    pub encoding: String,
    pub token: String,
}
