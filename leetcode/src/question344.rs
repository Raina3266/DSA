use test_strategy::proptest;

pub fn reverse_string(s: &mut Vec<char>) {
    if s.is_empty() {
        return;
    }
    let mut b = s.len() - 1;
    let mut a = 0;
    while a < s.len()/2 {

        (s[a], s[b]) = (s[b], s[a]);
        a += 1;
        b -= 1;
    }
}


#[test]
fn test() {
    let mut chars: Vec<_> = "hello".chars().collect();
    reverse_string(&mut chars);

    assert_eq!(chars, ['o', 'l', 'l', 'e', 'h']);
}

#[proptest]
fn reversing_a_string_twice_leaves_it_unchanged(chars: Vec<char>) {
    let mut chars_clone = chars.clone();

    reverse_string(&mut chars_clone);
    reverse_string(&mut chars_clone);

    assert_eq!(chars, chars_clone);
}
