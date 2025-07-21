use rust_learning::testing;

#[test]
fn it_adds() {
    let result = testing::add(2, 2);
    assert_eq!(result, 4);
}
