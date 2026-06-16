pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut ans = vec![];
    let mut potions = potions;
    potions.sort();
    for s in spells {
        ans.push(one_spell_for_potions(s, &potions, success))
    }
    ans
}
// 25
// 0,1,2
// 10,20,30
fn one_spell_for_potions(spell: i32, potions: &[i32], success: i64) -> i32 {
    let mut left = 0;
    let mut right = potions.len() - 1;

    loop {
        if right - left <= 1 {
            let product_max: i64 = (potions[right] as i64) * (spell as i64);
            let product_min: i64 = (potions[left] as i64) * (spell as i64);

            if product_max < success {
                return 0;
            } else if product_min >= success {
                return potions.len().try_into().unwrap();
            } else {
                return (potions.len() - right).try_into().unwrap();
            }
        }
        let mid = (left + right) / 2;
        println!("{left}");
        println!("{right}");
        let product: i64 = (potions[mid] as i64) * (spell as i64);
        if product >= success {
            right = mid;
        } else {
            left = mid;
        }
    }
}

#[test]
fn feature() {
    let spell = 27;
    let potions = &[
        5, 11, 11, 11, 17, 17, 17, 17, 18, 18, 19, 19, 19, 20, 20, 20, 22, 22, 22, 22, 22, 23, 24,
        24, 25, 25, 27, 27, 28, 28, 28, 28, 29, 29, 29, 30, 31, 32, 32, 32, 33, 33, 33, 34, 34, 35,
        35, 35, 36, 36, 36, 36, 36, 36, 37, 37, 37, 37, 38, 38, 38, 39, 39, 39, 40, 40, 40, 40, 40,
        40, 40, 40,
    ];
    let success = 135;
    assert_eq!(one_spell_for_potions(spell, potions, success), 72)
}
