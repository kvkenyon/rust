use rust_learning::testing;

mod common;

#[test]
fn it_adds() {
    common::setup();
    let result = testing::add(2, 2);
    assert_eq!(result, 4);
}
