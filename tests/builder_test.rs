#[cfg(test)]

extern crate opentdb;

/// This is a built-in limitation by the remote API.
///
/// - A Maximum of 50 Questions can be retrieved per call.
#[test]
fn builder_simple_panics_at_plus50_questions_test() {
    let result = std::panic::catch_unwind(|| {
        opentdb::simple_builder()
            .questions(51)
            .build();
    });
    assert!(result.is_err());
}

/// This is a built-in limitation by the remote API.
///
/// - A Maximum of 50 Questions can be retrieved per call.
#[test]
fn builder_strict_panics_at_plus50_questions_test() {
    let result = std::panic::catch_unwind(|| {
        opentdb::strict_builder()
            .questions(51)
            .build();
    });
    assert!(result.is_err());
}
