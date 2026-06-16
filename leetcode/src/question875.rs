pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut piles = piles;
    let h_usize: usize = h.try_into().unwrap();
    piles.sort();
    let length = piles.len();
    if length == h_usize {
        return piles[length - 1];
    }
    if piles.len() == 1 {
        
        if piles[0] % h == 0 {
            return piles[0] / h;
        } else {
            return piles[0] / h + 1;
        }
    }

    let mut left = 1;
    let mut right = piles[length - 1];
    loop {
        if right - left <= 1 {
            if calculate_hours_for_one_speed(&piles, left) <= h {
                return left;
            } else {
                return right;
            }
        }
        let midpoint = (right + left) / 2;

        if calculate_hours_for_one_speed(&piles, midpoint) <= h {
            right = midpoint;
        } else {
            left = midpoint;
        }
    }
}

fn calculate_hours_for_one_speed(piles: &[i32], speed: i32) -> i32 {
    let mut sum = 0;
    for p in piles {
        if p % speed == 0 {
            sum += p / speed;
        } else {
            sum += p / speed + 1;
        }
    }
    sum
}

#[test]
fn feature() {
    let piles = vec![
        332484035, 524908576, 855865114, 632922376, 222257295, 690155293, 112677673, 679580077,
        337406589, 290818316, 877337160, 901728858, 679284947, 688210097, 692137887, 718203285,
        629455728, 941802184,
    ];
    assert_eq!(min_eating_speed(piles, 823855818), 14)
}
