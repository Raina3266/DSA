pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
    let mut new_mass: i64 = mass.into();
    let mut new_asteroids: Vec<i64> = asteroids.into_iter().map(|x| x as i64).collect();
    new_asteroids.sort();
    if new_mass < new_asteroids[0] {
        return false;
    }
    let max: i64 = new_asteroids[new_asteroids.len() - 1];

    loop {
        if new_mass >= max {
            return true;
        }
        let biggest = new_asteroids.iter().enumerate().filter(|(_, n)| **n <= new_mass).max_by_key(|&(_, n)| n);
        if biggest.is_none() {
            return false;
        } else {
            let x: i64 = new_asteroids[0..=biggest.unwrap().0].iter().sum();
            new_mass += x;
            new_asteroids.drain(..biggest.unwrap().0 +1);
        }
    }
}

#[test]
fn feature() {
    let asteroids = vec![4,4,9,23];
    let mass = 5;
    assert!(!asteroids_destroyed(mass, asteroids));
}
