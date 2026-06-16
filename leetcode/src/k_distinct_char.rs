use std::collections::HashMap;


pub fn k_distinct_char(k: usize, s: String) -> String {
    let s_as_list: Vec<char> = s.chars().collect();

    let mut chars_in_window = HashMap::<char, usize>::new();
    let mut start = 0;
    let mut end = 0;
    let mut ans = 0;

    loop {
        // move a pointer
        if chars_in_window.len() <= k {
            let next_char = s_as_list[end + 1];
            if chars_in_window.contains_key(&next_char) {
                end += 1;
                *chars_in_window.get_mut(&next_char).unwrap() += 1;
            } else {
                start += 1;
                let old_char = s_as_list[start - 1];
                let mut_ref = chars_in_window.get_mut(&old_char).unwrap();
                *mut_ref -= 1;
                if *mut_ref == 0 {
                    chars_in_window.remove(&old_char);
                }
            }
        } else {
            end += 1;
            let next_char = s_as_list[end];
            *chars_in_window.entry(next_char).or_insert(0usize) += 1;
        }

        // update max
        let current = end - start;
        ans = std::cmp::max(ans, current);
    }

    // while start < s_as_list.len() && end < s_as_list.len() {
    //     *chars_in_window.entry(s_as_list[end]).or_insert(0) += 1;

    //     if chars_in_window.len() > k {
    //         *chars_in_window.get_mut(&s_as_list[start]).unwrap() -= 1;

    //     } else {
    //         end += 1;
    //     }


    // }
}