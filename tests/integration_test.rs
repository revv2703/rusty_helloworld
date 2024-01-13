use rusty_helloworld;
mod common;

// To run only the integration tests, use "cargo test --test integration_test"

#[test]
fn it_adds_two_integ() {
    common::setup();
    assert_eq!(4, rusty_helloworld::adds_two(2));
}

// Cannot directly test a binary crate(main.rs) with integration tests