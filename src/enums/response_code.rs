use std::fmt;

/// The API will always return a "Response Code",
/// this value can be used to determine whether or not your request was a success.
#[derive(Debug, PartialEq)]
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
    /// Unknown Code: We don't know how to map this value and we don't know what it is.
    /// Not part of the official API, only returned in ResponseCode::from(u8)
    Unknown = 100,
}

impl Default for ResponseCode {
    fn default() -> Self {
        ResponseCode::Success
    }
}

impl From<u8> for ResponseCode {
    fn from(data: u8) -> Self {
        match data {
            0 => ResponseCode::Success,
            1 => ResponseCode::NoResults,
            2 => ResponseCode::InvalidParameter,
            3 => ResponseCode::TokenNotFound,
            4 => ResponseCode::TokenEmpty,
            _ => ResponseCode::Unknown,
        }
    }
}

/// Source: https://github.com/serde-rs/serde-rs.github.io/blob/master/_src/enum-number.md
impl<'de> ::serde::Deserialize<'de> for ResponseCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResponseCode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("positive integer")
            }

            fn visit_u64<E>(self, value: u64) -> Result<ResponseCode, E>
                where
                    E: ::serde::de::Error,
            {
                // Rust does not come with a simple way of converting a
                // number to an enum, so use a big `match`.
                match value {
                    0 => Ok(ResponseCode::Success),
                    1 => Ok(ResponseCode::NoResults),
                    2 => Ok(ResponseCode::InvalidParameter),
                    3 => Ok(ResponseCode::TokenNotFound),
                    4 => Ok(ResponseCode::TokenEmpty),
                    _ => Err(E::custom(format!("unknown ResponseCode with value: {}", value))),
                }
            }
        }

        // Deserialize the enum from a u64.
        deserializer.deserialize_u64(Visitor)
    }
}
