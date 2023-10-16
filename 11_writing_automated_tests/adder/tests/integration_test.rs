use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(adder::adds_two(2), 4);
}
