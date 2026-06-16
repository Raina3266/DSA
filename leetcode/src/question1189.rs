

pub fn max_number_of_balloons(text: String) -> i32 {
    use std::collections::HashMap;
    let balloon = String::from("balon");
    let mut list: Vec<i32> = vec![];
    let mut map:HashMap<char, i32> = HashMap::new();
    for s in text.chars(){
        *map.entry(s).or_insert(0) += 1
    }
    for i in balloon.chars(){
        list.push(*map.get(&i).unwrap_or(&0));
        
    }
    list[2] /= 2;
    list[3] /= 2;

    list.into_iter().min().unwrap()
}