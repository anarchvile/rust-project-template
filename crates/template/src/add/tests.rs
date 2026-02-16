//! Unit tests for addition.

/// Unit tests for [`super::add`].
#[test]
fn add() {
    assert_eq!(super::add(0, 0), 0);
    assert_eq!(super::add(1, 3), 4);
    assert_eq!(super::add(-3, 5), 2);
    assert_eq!(super::add(2, -3), -1);
    assert_eq!(super::add(-4, -7), -11);
    assert_eq!(super::add(i32::MAX, i32::MAX), i32::MAX);
    assert_eq!(super::add(i32::MIN, i32::MIN), i32::MIN);
}
