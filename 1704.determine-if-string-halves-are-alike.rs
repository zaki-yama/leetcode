/*
 * @lc app=leetcode id=1704 lang=rust
 *
 * [1704] Determine if String Halves Are Alike
 */

// @lc code=start
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let (a, b) = s.split_at(s.as_bytes().len() / 2);
        let a_count = Solution::count_vowel(a);
        let b_count = Solution::count_vowel(b);

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
// @lc code=end
