/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut result = 0;
        let mut k = 0;
        while k <= n / 2 {
            // for k in 0..n {
            let digit = (n - 2 * k) + k;
            println!("digit: {}", digit);
            result += Solution::c(digit, k);
            println!("result: {}", result);
            k += 1;
        }
        result
    }

    fn c(n: i32, k: i32) -> i32 {
        if k == 0 || k == n {
            return 1;
        }
        Solution::c(n - 1, k - 1) * n / k
    }
}
// @lc code=end
