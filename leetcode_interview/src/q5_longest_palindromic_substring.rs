// need to do it again
pub fn longest_palindrome(s: String) -> String {
    if s.len() <= 1 {
        return s;
    }
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut end = 0;
    let mut max_len = 0;

    for index in 0..bytes.len() {
        // 1. odd case
        let mut left = index;
        let mut right = index;
        while right < bytes.len() && bytes[left] == bytes[right] {
            if right - left + 1 > max_len {
                start = left;
                end = right;
                max_len = right - left + 1;
            }
            if left == 0 {
                break;
            }
            left -= 1;
            right += 1;
        }

        // 2. even case
        let mut left = index;
        let mut right = index + 1;
        while right < bytes.len() && bytes[left] == bytes[right] {
            if right - left + 1 > max_len {
                start = left;
                end = right;
                max_len = right - left + 1;
            }
            if left == 0 {
                break;
            }
            left -= 1;
            right += 1;
        }
    }

    String::from_utf8(bytes[start..=end].to_vec()).unwrap()
}



pub fn longest_palindrome_my_answer(s: String) -> String {
    if s.len() == 1 {
        return s;
    }
    let s = s.as_bytes();
    if s.len() == 2 && s[0] == s[1] {
        return String::from_utf8(s.to_vec()).unwrap();
    }

    let (mut index, mut count) = (0, 0);
    for n in 0..s.len() {
        let mut middle_x: usize = 0;
        while n >= middle_x && n + middle_x < s.len() && s[n - middle_x] == s[n + middle_x] {
            middle_x += 1
        }
        let mut same_letter_x: usize = 0;
        while n + middle_x < s.len() && s[n] == s[n + same_letter_x] {
            same_letter_x += 1;
        }
        if middle_x * 2 - 1 > count && middle_x * 2 - 1 > same_letter_x {
            index = n - middle_x + 1;
            count = middle_x * 2 - 2;
        }
        if same_letter_x > count && same_letter_x > middle_x * 2 - 1 {
            index = n;
            count = same_letter_x - 1;
        }
    }
    let new_s = &s[index..=index + count];
    String::from_utf8(new_s.to_vec()).unwrap()
}

#[test]
fn feature() {
    let s = "bb".to_string();
    let output = "bb".to_string();
    assert_eq!(longest_palindrome(s), output)
}

#[test]
fn feature2() {
    let s = "cbbbbbd".to_string();
    let output = "bbbbb".to_string();
    assert_eq!(longest_palindrome(s), output)
}
