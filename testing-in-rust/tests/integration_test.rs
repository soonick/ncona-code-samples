use testing;

#[test]
fn integration_test() {
    let result = testing::add(2, 2);
    assert_eq!(result, 4);
}
