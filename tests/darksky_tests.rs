extern crate weather;

mod common;

#[test]
fn check_for_api_key() {
    common::setup();

    assert!(std::env::var("DARKSKY_KEY").ok().is_some());
}
