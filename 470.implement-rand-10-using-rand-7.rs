/*
 * @lc app=leetcode id=470 lang=rust
 *
 * [470] Implement Rand10() Using Rand7()
 */

// @lc code=start
/**
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

impl Solution {
    pub fn rand10() -> i32 {
        loop {
            let row = rand7();
            let col = rand7();
            let idx = col + (row - 1) * 7;

            if idx <= 40 {
                return 1 + (idx - 1) % 10;
            }
        }
    }
}
// @lc code=end
