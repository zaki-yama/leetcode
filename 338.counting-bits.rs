/*
 * @lc app=leetcode id=338 lang=rust
 *
 * [338] Counting Bits
 */

// @lc code=start
impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut x: Vec<i32> = vec![0; (num + 1) as usize];
        for i in 1..num + 1 {
            x[i as usize] = x[(i & (i - 1)) as usize] + 1;
        }

        x
    }
}
// @lc code=end
