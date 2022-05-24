use private_fn_integration_test;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, private_fn_integration_test::add_two(2));
}
