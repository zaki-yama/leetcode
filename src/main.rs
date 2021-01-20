#[test]
fn test() {
    assert_eq!(true, Solution::halves_are_alike(String::from("book")));
    assert_eq!(false, Solution::halves_are_alike(String::from("textbook")));
    assert_eq!(
        false,
        Solution::halves_are_alike(String::from("MerryChristmas"))
    );
    assert_eq!(true, Solution::halves_are_alike(String::from("AbCdEfGh")));
}

fn main() {
    // println!("{}", Solution::halves_are_alike(String::from(raw_str)));
}

struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let (a, b) = s.split_at(s.as_bytes().len() / 2);
        let a_count = Solution::count_vowel(a);
        let b_count = Solution::count_vowel(b);
        println!("({}, {}) = ({}, {})", a, b, a_count, b_count);
        a_count == b_count
    }

    pub fn count_vowel(s: &str) -> usize {
        s.as_bytes()
            .iter()
            .filter(|ch| match ch {
                b'a' | b'A' | b'i' | b'I' | b'u' | b'U' | b'e' | b'E' | b'o' | b'O' => {
                    return true;
                }
                _ => false,
            })
            .count()
    }
}
