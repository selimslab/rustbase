//! Integration test: imports the crate by name, proving the lib/bin split works.

// `unwrap`/`expect` are how a test asserts; the restriction lints target library code.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use mars::{Error, Greeting};

#[test]
fn library_is_reachable_from_an_external_test_binary() {
    let greeting = Greeting::new("Mars").expect("`Mars` is a valid name");
    assert_eq!(greeting.render(), "Hello, Mars!");
}

#[test]
fn blank_names_are_rejected_across_the_crate_boundary() {
    assert!(matches!(Greeting::new(""), Err(Error::BlankName)));
}
