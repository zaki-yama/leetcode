/*
 * @lc app=leetcode id=279 lang=rust
 *
 * [279] Perfect Squares
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut ans = vec![std::i32::MAX - 1; (n + 1) as usize];
        ans[0] = 0;
        for i in 1..=n {
            let mut j = 1;
            while j * j <= i {
                ans[i as usize] = cmp::min(ans[i as usize], ans[(i - j * j) as usize] + 1);
                j += 1;
            }
        }
        ans[n as usize]
    }
}
// @lc code=end
