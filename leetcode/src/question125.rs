fn is_palindrome(s: String) -> bool {
    let mut s = s;
    s.make_ascii_lowercase();

    let filtered_string: Vec<_> = s.chars().filter(|x| x.is_alphanumeric()).collect();
    let length = filtered_string.len();

    for index in 0..length {
        
        if filtered_string[index] == filtered_string[length - index -1] {

        } else {
            return false
        }
    }
    true
}

#[test]
fn genius_raina() {
    assert_eq!(is_palindrome("jhif,shsiuw".into()), false);
    assert_eq!(is_palindrome("A man, a plan, a canal: Panama".into()), true);
    assert_eq!(is_palindrome("".into()), true); 
}