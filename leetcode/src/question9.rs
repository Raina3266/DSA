
pub fn palindrome(x: i32) -> bool {
    let y = x.to_string();
    let z:String = y.chars().rev().collect();
    z == y
}


pub fn check_if_palindrome(s: String) -> bool {
    let list: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        if list[left] != list[right]{
            return false;
        }
    left += 1;
    right -= 1;
    }
    true
}

#[test]
fn example_1() {
    let s = "asdfdsty";
    assert_eq!(check_if_palindrome(s.to_string()),false);
}