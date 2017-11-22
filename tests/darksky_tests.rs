extern crate nimbus;

mod common;

#[test]
#[ignore]
fn check_for_api_key() {
    common::setup();

    assert!(std::env::var("DARKSKY_KEY").ok().is_some());
}
