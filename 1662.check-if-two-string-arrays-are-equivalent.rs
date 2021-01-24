/*
 * @lc app=leetcode id=1662 lang=rust
 *
 * [1662] Check If Two String Arrays are Equivalent
 */

// @lc code=start
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.join("") == word2.join("")
    }
}
// @lc code=end
