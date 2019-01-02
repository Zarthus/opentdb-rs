/// The API will always return a "Response Code",
/// this value can be used to determine whether or not your request was a success.
#[derive(Deserialize, Debug)]
pub enum ResponseCode {
    /// Code 0: Returned results successfully.
    Success = 0,
    /// Code 1: Could not return results. The API doesn't have enough questions for your query.
    /// (Ex. Asking for 50 Questions in a Category that only has 20.)
    NoResults = 1,
    /// Code 2: Contains an invalid parameter.
    /// Arguments passed in aren't valid. (Ex. Amount = Five)
    InvalidParameter = 2,
    /// Code 3: Session Token does not exist.
    TokenNotFound = 3,
    /// Code 4: Session Token has returned all possible questions for the specified query.
    /// Resetting the Token is necessary.
    TokenEmpty = 4,
}

impl Default for ResponseCode {
    fn default() -> Self {
        ResponseCode::Success
    }
}
