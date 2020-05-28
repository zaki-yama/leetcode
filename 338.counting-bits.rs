/*
 * @lc app=leetcode id=338 lang=rust
 *
 * [338] Counting Bits
 */

// @lc code=start
impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut x: Vec<i32> = Vec::with_capacity((num as usize) + 1);
        for i in 0..num + 1 {
            x.push(i.count_ones() as i32);
        }

        x
    }
}
// @lc code=end
