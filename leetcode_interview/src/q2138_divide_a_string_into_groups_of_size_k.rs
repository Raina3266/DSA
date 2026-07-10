pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    let mut s = s.chars().collect::<Vec<char>>();
    let k = k as usize;
    s.resize(s.len() + k - s.len() % k, fill);
    s.chunks(k).map(|c| c.iter().collect()).collect()
}
