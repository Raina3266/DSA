pub fn remove_duplicates(s: String) -> String {
    let mut ans: Vec<char> = vec![];
    for c in s.chars() {
        if ans.is_empty() || &c != ans.last().unwrap(){
            ans.push(c);  
        } else {
            ans.pop();
        }        
    }
    ans.iter().collect()
}