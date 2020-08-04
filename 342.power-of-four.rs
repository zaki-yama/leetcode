/*
 * @lc app=leetcode id=342 lang=rust
 *
 * [342] Power of Four
 */

// @lc code=start
impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        if num == 1 {
            return true;
        }
        let mut a = num;
        let mut amari = 1;
        loop {
            println!("{}", a);
            if a < 4 {
                break;
            }
            amari = a % 4;
            if amari != 0 {
                return false;
            }
            a /= 4;
        }
        amari == 0 && a == 1
    }
}
// @lc code=end
