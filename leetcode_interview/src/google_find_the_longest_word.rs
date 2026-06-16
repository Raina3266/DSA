// Given a string S and a set of words D, find the longest word in D that is a subsequence of S.

// Word W is a subsequence of S if some number of characters, possibly zero, can be deleted from S to form W, without reordering the remaining characters.

// Note: D can appear in any format (list, hash table, prefix tree, etc.

// For example, given the input of S = "abppplee" and D = {"able", "ale", "apple", "bale", "kangaroo"} the correct output would be "apple"

// The words "able" and "ale" are both subsequences of S, but they are shorter than "apple".
// The word "bale" is not a subsequence of S because even though S has all the right letters, they are not in the right order.
// The word "kangaroo" is the longest word in D, but it isn't a subsequence of S.
// Learning objectives: this question gives you the chance to practice with algorithms and data structures. It’s also a good example of why careful analysis 
// for Big-O performance is often worthwhile, as is careful exploration of common and worst-case input conditions.

fn find_the_longest_words<T: IntoIterator<Item = String>>(target: String, set_of_words: T) -> String {
    let mut max_length = 0;
    let mut bigger_chars = target.chars();
    let len = bigger_chars.clone().count();
    for word in set_of_words {
        if word.len() > max_length {
            let mut left_pointer = 0;
            for char in word.chars() {
                for i in left_pointer..=len {
                    if bigger_chars.nth(i) == Some(char) {
                        left_pointer = i;
                            continue;
                    }
                }
            }
        }
    }
    todo!()
}

fn is_subsequence<T: IntoIterator<Item = String>>(target: &str, word: &str) -> bool {
    let mut long_pointer = 0;
    let target_len = target.len();
    let mut short_pointer = 0;
    let word_len = word.len();
    
    todo!()
}