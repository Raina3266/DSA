// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.


pub fn is_valid(s: String) -> bool { 
    let mut stack = vec![];
    for char in s.chars() {
        if char == '(' || char == '[' || char == '{' {
            stack.push(char);
        } else {
            let Some(last_char) = stack.pop() else {
                return false;
            };
            let is_match = match last_char {
                '(' => char == ')',
                '[' => char == ']',
                '{' => char == '}',
                _ => panic!()
            };
            if !is_match {
                return false;
            }
        }
    }
    stack.is_empty()
}