use std::collections::HashMap;

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let mut map: HashMap<char, Vec<char>> = HashMap::new();
    let two = ('2', vec!['a', 'b', 'c']);
    let three = ('3', vec!['d', 'e', 'f']);
    let four = ('4', vec!['g', 'h', 'i']);
    let five = ('5', vec!['j', 'k', 'l']);
    let six = ('6', vec!['m', 'n', 'o']);
    let seven = ('7', vec!['p', 'q', 'r', 's']);
    let eight = ('8', vec!['t', 'u', 'v']);
    let nine = ('9', vec!['w', 'x', 'y', 'z']);

    map.insert(two.0, two.1);
    map.insert(three.0, three.1);
    map.insert(four.0, four.1);
    map.insert(five.0, five.1);
    map.insert(six.0, six.1);
    map.insert(seven.0, seven.1);
    map.insert(eight.0, eight.1);
    map.insert(nine.0, nine.1);

    let mut ans = vec![];
    let digits: Vec<char> = digits.chars().collect();
    let mut nums: Vec<Vec<char>> = vec![];
    for d in digits {
        nums.push(map.get(&d).unwrap().to_vec());
    }
    for one in combine(nums) {
        let combined: String = one.iter().collect();
        ans.push(combined);
    }
    ans
}

fn combine(nums: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut ans: Vec<Vec<char>> = vec![];
    if nums.len() == 1 {
        for n in &nums[0] {
            let inner = vec![*n];
            ans.push(inner);
        }
        return ans;
    }
    let (head, tail) = nums.split_first().unwrap();

    for c in combine(tail.to_vec()) {
        for h in head {
            let mut c = c.clone();
            c.insert(0, *h);
            ans.push(c.clone());
        }
    }
    ans
}

#[test]
fn feature() {
    let nums = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];
    assert_eq!(combine(nums), vec![vec!['a'], vec!['b'], vec!['c']]);
}
