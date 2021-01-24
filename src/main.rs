#[test]
fn test() {
    let word1 = vec![String::from("ab"), String::from("c")];
    let word2 = vec![String::from("a"), String::from("bc")];
    assert_eq!(true, Solution::array_strings_are_equal(word1, word2));
}

fn main() {
    // println!("{}", Solution::halves_are_alike(String::from(raw_str)));
}

struct Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.join("") == word2.join("")
    }
}
