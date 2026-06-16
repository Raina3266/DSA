// Check whether a number is a palindrome (base 10) - i.e. it is the same forward and backward
// e.g. 121, 1, 0, are all palindromes,
// 23, 123 are not

fn is_palindrome(x: u32) -> bool {
    let mut digits = vec![];
    let mut x = x;

    if x == 0 {
        return true;
    }
    while x != 0 {
        let last_digit = x % 10;
        digits.push(last_digit);
        x /= 10;
    }

    let mut start = 0;
    let mut end = digits.len() - 1;

    while start < end {
        if digits[start] != digits[end] {
            return false;
        } else {
            start += 1;
            end -= 1;
        }
    }
    true
}

fn is_palindrome_2(mut x: u32) -> bool {
    let mut rev_x = 0;
    let copy_x = x;

    while x != 0 {
        let last_digit = x % 10;
        rev_x *= 10;
        rev_x += last_digit;
        x /= 10;
    }
    rev_x == copy_x
}

enum Json {
    Null,
    Bool(bool),
    // ...
}

