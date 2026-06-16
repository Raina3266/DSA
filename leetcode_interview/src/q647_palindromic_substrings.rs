// no need to revisit
pub fn count_substrings(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut count = 0;
    for n in 0..bytes.len() {
        let mut right = bytes.len();
        while right > n {
            right -= 1;
            if is_palindrome(&bytes[n..=right]) {
                count += 1;
            }
        }
    }
    count
}

fn is_palindrome(s: &[u8]) -> bool {
    s.iter().zip(s.iter().rev()).all(|(l, r)| l == r)
}

#[test]
fn test() {
    let input = String::from("abc");
    assert_eq!(count_substrings(input), 3);
}
