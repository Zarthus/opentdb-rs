# opentdb-rs

[![Build Status](https://travis-ci.org/Zarthus/opentdb-rs.svg?branch=master)](https://travis-ci.org/Zarthus/opentdb-rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://meritbadge.herokuapp.com/opentdb)](https://crates.io/crates/opentdb)
[![Released API docs](https://docs.rs/opentdb/badge.svg)](https://docs.rs/opentdb)
[![dependency status](https://deps.rs/repo/github/zarthus/opentdb-rs/status.svg)](https://deps.rs/repo/github/zarthus/opentdb-rs)
[![codecov](https://codecov.io/gh/zarthus/opentdb-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/zarthus/opentdb-rs/branch/master/)

An implementation of the Open Trivia Database API in Rust <https://opentdb.com>

### Install

```bash
cargo add opentdb
```

### Usage

```rust
extern crate opentdb;

use opentdb::api_request::ApiRequest;
use opentdb::api_response::ApiResponse;

use opentdb::enums::category::Category;
use opentdb::enums::difficulty::Difficulty;
use opentdb::enums::question_type::QuestionType;
use opentdb::enums::encoding::Encoding;

fn main() {
    // Start a new session so that the service remains the questions we've received already.
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
```

### Limitations

- Only 1 Category can be requested per API Call. To get questions from any category, don't specify a category.
- A Maximum of 50 Questions can be retrieved per call.

### Missing

The following API lookups are not implemented:

Category Lookup: Returns the entire list of categories and ids in the database.

- https://opentdb.com/api_category.php

Category Question Count Lookup: Returns the number of questions in the database, in a specific category.

- https://opentdb.com/api_count.php?category=CATEGORY_ID_HERE

Global Question Count Lookup: Returns the number of ALL questions in the database. Total, Pending, Verified, and Rejected.

- https://opentdb.com/api_count_global.php

### License and Acknowledgements

OpenTDB itself is licensed under the [MIT license](LICENSE)

OpenTDB depends on `serde` (for (de)serialization) and `reqwest` (for sending HTTP requests).
