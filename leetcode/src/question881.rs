#[allow(clippy::collapsible_else_if)]
pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let mut ans = 0;
    let mut people = people;
    people.sort();

    // 7 2
    // 2,4,6,/ 8,10
    let cut_off = people.iter().enumerate().filter(|(_, n)| **n <= limit).max_by_key(|(_, n)| **n).unwrap();
    
    let one_person_boats = people.len() - 1 - cut_off.0;
    ans += one_person_boats;
    let mut smaller = people[0..=cut_off.0].to_vec();
    while smaller.len() <= 1 {
        if smaller[0] + smaller[smaller.len() - 1] <= limit {
            smaller.remove(0);
            smaller.remove(smaller.len() - 1);
            ans += 1;
        } else {
            smaller.remove(smaller.len() - 1);
            ans += 1;
        }
    }
    if smaller.is_empty() {
        ans as i32
    } else {
        (ans + 1) as i32
    }
}
