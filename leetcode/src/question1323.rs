pub fn maximum69_number(num: i32) -> i32 {
    let mut new_num = num;
    let mut nums: Vec<i32> = vec![];
    while new_num != 0 {
        nums.push(new_num % 10);
        new_num /= 10;
    }
    println!("{nums:?}");
    if nums.iter().all(|n| *n == 9) {
        return num;
    }
    nums.reverse();

    let i = nums.iter().position(|&x| x == 6).unwrap();
    println!("{i}");
    nums[i] = 9;
    println!("{nums:?}");

    turn_nums_into_num(nums)
}

fn turn_nums_into_num(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let length = nums.len();
    for i in 0..nums.len() {
        ans += nums[i] * 10i32.pow((length - i - 1).try_into().unwrap());
    }
    ans
}

#[test]
fn feature() {
    assert_eq!(maximum69_number(9966), 9996);
}
