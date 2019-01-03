# Changelog

## 1.0.0

This release marks the first stable version of opentdb-rs.

This is a breaking release.

### Breaking

- `opentdb::session_new()` now requires a parameter: `base_url: Option<&str>`
- `opentdb::session_reset()` now requires a second parameter: `base_url: Option<&str>`

It is suggested to just pass `None` to this if you are upgrading, unless you wish to make
use of a custom url.

### Features

- The Base URL is now configurable
- Examples have been added
- Enum ResponseCode is now properly utilized

### Bug fixes

- lib: All cargo warnings are fixed
- lib: to_url() now properly urlencodes items
- test: Integration tests now mock responses
- cargo: fix crates.io link for Travis due to case sensitivity

### General

- More tests have been added
- Optimized README, documentation, etc.
- CI has been sped up

### Overview

All fixed issues can be seen here:
[issues with milestone: v1.0.0](https://github.com/Zarthus/opentdb-rs/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aclosed+milestone%3Av1.0.0)

## 0.1.1

This is a non-breaking release, the API has not been changed.

Improvements to:

- code coverage (more tests)
- code style (editorconfig introduction)
- code (removes cargo warnings)
- documentation (readme, docblocks, Cargo.toml fixes)
- infrastructure (ci setup)