pub fn is_power_of_two(n: u32) -> bool {
    if n == 1 {
        return true;
    }

    if n % 2 == 0 {
        is_power_of_two(n / 2)
    } else {
        false
    }
}

#[test]
fn foo() {
    assert_eq!(is_power_of_two(64), true);
    assert_eq!(is_power_of_two(32), true);
    assert_eq!(is_power_of_two(128), true);
    assert_eq!(is_power_of_two(55), false);
    assert_eq!(is_power_of_two(127), false);
}
