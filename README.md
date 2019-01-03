# opentdb-rs

[![Build Status](https://travis-ci.org/Zarthus/opentdb-rs.svg?branch=master)](https://travis-ci.org/Zarthus/opentdb-rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://meritbadge.herokuapp.com/opentdb)](https://crates.io/crates/opentdb)
[![Released API docs](https://docs.rs/opentdb/badge.svg)](https://docs.rs/opentdb)
[![dependency status](https://deps.rs/repo/github/zarthus/opentdb-rs/status.svg)](https://deps.rs/repo/github/zarthus/opentdb-rs)
[![codecov](https://codecov.io/gh/zarthus/opentdb-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/zarthus/opentdb-rs/branch/master/)

An implementation of the [Open Trivia Database](https://opentdb.com) API in Rust

### Install

```bash
cargo add opentdb
```

### Usage

For usage, review the [examples here](examples): [examples/01_base_usage.rs](examples/01_base_usage.rs)

### Limitations

- Only 1 Category can be requested per API Call. To get questions from any category, don't specify a category.
- A Maximum of 50 Questions can be retrieved per call.

### License and Acknowledgements

opentdb-rs itself is licensed under the [MIT license](LICENSE), opentdb-rs is not affiliated with the website or API it is using.

opentdb-rs depends on `serde` (for (de)serialization) and `reqwest` (for sending HTTP requests). It also uses a dev-dependency `mockito` for integration tests.

The Open Trivia Database is a collection of user-contributed trivia questions. This project would not be
possible without it.

All data provided by their API is available under the [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/)
as is written [on the Open Trivia Database website](https://opentdb.com/api_config.php).