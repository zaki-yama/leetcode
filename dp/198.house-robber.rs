/*
 * @lc app=leetcode id=198 lang=rust
 *
 * [198] House Robber
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut dp = vec![0; nums.len() + 1];

        dp[0] = 0;
        dp[1] = nums[0];

        for i in 1..nums.len() {
            dp[i + 1] = cmp::max(dp[i - 1] + nums[i], dp[i]);
        }
        dp[nums.len()]
    }
}
// @lc code=end
