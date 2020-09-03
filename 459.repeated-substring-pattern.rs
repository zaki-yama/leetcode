/*
 * @lc app=leetcode id=459 lang=rust
 *
 * [459] Repeated Substring Pattern
 */

// @lc code=start
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let a = String::from(&s) + &s;

        println!("{}", a);
        println!("{}", &a[1..a.len()]);
        a[1..a.len() - 1].find(&s).is_some()
    }
}
// @lc code=end
