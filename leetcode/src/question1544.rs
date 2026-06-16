pub fn make_good(s: String) -> String {
    let mut ans: Vec<char> = vec![];

    for c in s.chars() {
        if !ans.is_empty()
            && c.is_uppercase()
            && ans.last().unwrap().is_lowercase()
            && c.eq_ignore_ascii_case(ans.last().unwrap())
        {
            ans.pop().unwrap();
        } else if !ans.is_empty()
            && c.is_lowercase()
            && ans.last().unwrap().is_uppercase()
            && ans.last().unwrap().eq_ignore_ascii_case(&c)
        {
            ans.pop().unwrap();
        } else {
            ans.push(c);
        }
    }
    ans.iter().collect()
}
