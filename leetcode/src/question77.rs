pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    if n == 1 {
        for i in 1..=n {
            ans.push(vec![i]);
        }
    }
    let list_with_k_length: Vec<i32> = vec![2; k as usize];
    println!("{list_with_k_length:?}");
    let available_nums: Vec<i32> = (1..=n).collect();
    fix_length_combine(list_with_k_length, available_nums)
}

fn fix_length_combine(list: Vec<i32>, nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = vec![];
    let mut list = list;
    

    if list.len() == 1 {
        for n in nums {
            let one_item = vec![n];
            ans.push(one_item);
        }
        return ans;
    }

    let (head, tail) = list.split_first().unwrap();

    let mut nums = nums;
    while let Some(pop) = nums.pop() {
        let combinations = fix_length_combine(tail.to_vec(), nums.clone());
        for one in combinations {
            let mut one = one;
            one.push(pop);
            ans.push(one.clone());
        }
    }
    ans
}

#[test]
fn feature() {
    let result = vec![
        vec![1, 2],
        vec![1, 3],
        vec![1, 4],
        vec![2, 3],
        vec![2, 4],
        vec![3, 4],
    ];
    assert_eq!(combine(4, 2), result)
}
