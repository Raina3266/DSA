
// Input: haystack = "sadbutsad", needle = "sad"
// Output: 0
// Explanation: "sad" occurs at index 0 and 6.
// The first occurrence is at index 0, so we return 0.



pub fn str_str(haystack: String, needle: String) -> i32 {
    for i in 0..=haystack.len() {
        if haystack[i..].starts_with(&needle) {
            return i as i32;
        } 
    }
    -1
}

pub fn str_str_2(haystack: String, needle: String) -> i32 {
    let h_vec: Vec<char> = haystack.chars().collect();
    let n_vec: Vec<char> = needle.chars().collect();
    let h_len = haystack.len();
    let n_len = needle.len();

    if n_len <= h_len {
        for i in 0..= h_len - n_len {
            if h_vec[i..i + n_len] == n_vec[..] {
                return i as i32;
            } 
        }
        -1
    } else {
        -1
    }
}