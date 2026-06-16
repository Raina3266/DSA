pub fn simplify_path(path: String) -> String {
    let segments: Vec<String> = path.split("/").map(|x| x.to_owned()).collect();
    let mut ans: Vec<String> = vec![];
    for s in segments.into_iter(){
        if !ans.is_empty() && s == ".." {
            ans.pop().unwrap();
        } else if s != "." && s != "" && s != ".." {
            ans.push(s);
        }
    }
    if ans.is_empty() {
        return "/".to_string();
    } else {
        ans.iter().map(|s| format!("/{s}")).collect()
    }
}
