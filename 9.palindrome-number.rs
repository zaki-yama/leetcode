/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string().chars().rev().eq(x.to_string().chars())
        // if x < 0 || (x % 10 == 0 && x != 0) {
        //     return false;
        // }

        // let (mut x, mut rev) = (x, 0);
        // while x > rev {
        //     rev = rev * 10 + x % 10;
        //     x /= 10;
        // }
        // x == rev || x == rev / 10
    }
}
// @lc code=end
