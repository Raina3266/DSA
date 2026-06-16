pub fn backspace_compare(s: String, t: String) -> bool {
    let mut ans_s:Vec<char> = vec![];
    let mut ans_t:Vec<char> = vec![];

    for ss in s.chars(){
        if ss == '#' {
            if !ans_s.is_empty(){
                ans_s.pop().unwrap();
            } 
        } else {
            ans_s.push(ss);
        }
    }
    for tt in t.chars(){
        if tt == '#' {
            if !ans_t.is_empty(){
                ans_t.pop().unwrap();
            } 
        } else {
            ans_t.push(tt);
        }
    }
    ans_s == ans_t
}