
pub fn is_subsequence(little: String, big: String) -> bool {
    let mut little = little.chars();
    let mut big = big.chars();
    
    #[allow(unused_labels)]
    'big: loop {
        let Some(target) = little.next() else {
            return true;
        };

        'small: loop {
            let Some(current) = big.next() else {
                return false;
            };

            if current == target {
                break 'small;
            } 
        }
    }
}