pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut big_nums: Vec<i32> = vec![];
    let mut small_nums = vec![];

    if m <= n {
        small_nums = (1..m).collect();
        for mm in 0..m - 1 {
            big_nums.push(n + mm);
        }
    } else {
        small_nums = (1..n).collect();
        for nn in 0..n - 1 {
            big_nums.push(m + nn);
        }
    }

    let mut remaining = vec![];
    
    for s in small_nums {
        if !change_big_nums(&mut big_nums, s) {
            remaining.push(s);
        }
    }

    let x: u128 = big_nums.iter().map(|x| *x as u128).product();
    let y: u128 = remaining.iter().map(|x| *x as u128).product();
    (x/y).try_into().unwrap()
}

fn change_big_nums(big_nums: &mut Vec<i32>, num: i32) -> bool {
    for b in big_nums.iter_mut() {
        if *b % num == 0 {
            *b /= num;
            return true;
        }
    }
    false
}

pub fn unique_path_two(m: i32, n: i32) -> i32 {
    if m == 1 || n == 1 {
        return 1;
    }

    unique_path_two(m - 1, n) + unique_path_two(m, n -1)
}


#[test]
fn feature() {
    assert_eq!(unique_path_two(23, 12), 193536720);
}

