// TODO: Fix bug #3
// /// Handle the response code.
// extern crate opentdb;

// use opentdb::api_request::ApiRequest;
// use opentdb::api_response::ApiResponse;
// use opentdb::enums::response_code::ResponseCode;

// fn main() {
//     // This token should have been extensively used before, but
//     // for purposes of demonstration we'll just create a new one.
//     let token: String = opentdb::session_new(None)
//         .expect("New Session Creation failed")
//         .token;

//     let rq: ApiRequest = opentdb::builder().token(token).build();
//     // This will still work, because when the ResponseCode is non-zero (ResponseCode::Success),
//     // the response will just be an empty JSON array, and this perfectly decodable by ApiResponse.
//     let rs: ApiResponse = opentdb::send_and_parse(rq).unwrap();

//     match rs.response_code {
//         ResponseCode::Success => {
//             // Nothing to do but deal with the Api Response.
//         },
//         ResponseCode::NoResults => {
//             // Retry the request with fewer questions or a less specific category.
//             // You can also try calling `opentdb::session_reset(token);` to allow for
//             // more questions to be returned.
//         },
//         ResponseCode::InvalidParameter => {
//             // Most probably one of the folowing cases is true:
//             //
//             // (1) The API has been updated and this library is no longer compatible with it, report as a bug.
//             // (2) The library just flat out has a bug in it, and it needs to be fixed.
//             // (3) Something unknown has happened.
//             //
//             // Whatever it is, we can't really recover from this. You can panic!() or deal with this however you see fit.
//             panic!("Library encountered an error.");
//         },
//         ResponseCode::TokenNotFound => {
//             // Session Tokens only maintain active for 6 hours after last use. You've probably
//             // kept it stored for too long. You will have to request a new token.
//             let token: String = opentdb::session_new();
//             // Store token somewhere.
//         },
//         ResponseCode::TokenEmpty => {
//             // We need to retry the request after this.
//             opentdb::session_reset(token);
//             // Retry the request with the same token.
//         }
//     }

//     // You don't *NEED* to deal with the Response Code other than ResponseCode::TokenEmpty,
//     // and even this is a stretch if you a very basic app that does not filter out much
//     // (because OpenTDB has >3000 items, you will not run out any time soon.)
//     //
//     // the API will return an empty array, and if there's nothing in there, there's no questions left.
//     //
//     // You may also choose to look at `rs.results.len()`
//     //
//     // The following will still just entirely work:
//     //
//     // for results in rs.results {
//     //     println!("  [{}] Question: {}", results.category, results.question);
//     // }

//     println!("Response Code: {:?}", rs.response_code);
// }
fn main() {}
