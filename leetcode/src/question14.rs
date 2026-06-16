
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut current_prefix: &str = &strs[0];

    while !current_prefix.is_empty() {
        if all_start_with(current_prefix, &strs) {
            return current_prefix.to_string();
        } else {
            current_prefix = &current_prefix[0..current_prefix.len() - 1];
        }
    }
    current_prefix.to_string()
}

pub fn all_start_with(prefix: &str, strs: &[String]) -> bool {
    for single_string in strs {
        let boo = single_string.starts_with(prefix);
        if !boo {
            return false;
        }
    }
    true
}