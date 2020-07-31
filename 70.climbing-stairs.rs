/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut dp = (1, 2);
        for k in 3..=n {
            dp = (dp.1, dp.0 + dp.1);
        }
        dp.1
    }
}
// @lc code=end
