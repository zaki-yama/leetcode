/*
 * @lc app=leetcode id=96 lang=rust
 *
 * [96] Unique Binary Search Trees
 */

// @lc code=start
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;

        for k in 2..=n {
            for i in 0..k {
                dp[k as usize] += dp[i as usize] * dp[(k - i - 1) as usize];
            }
        }
        dp[n as usize]
    }
}
// @lc code=end
