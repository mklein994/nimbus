extern crate nimbus;

mod common;

#[test]
#[ignore]
fn check_for_api_key() {
    common::setup();

    assert!(std::env::var("DARKSKY_KEY").ok().is_some());
}

#[test]
fn test_api_response() {
    let m = common::get_mock();
}
