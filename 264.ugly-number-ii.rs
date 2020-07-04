/*
 * @lc app=leetcode id=264 lang=rust
 *
 * [264] Ugly Number II
 */

// @lc code=start
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        use std::cmp::min;
        if n <= 0 {
            return 0;
        }
        let mut ans = 1;
        let mut ugly = Vec::new();
        ugly.push(1);

        let mut two_index = 0;
        let mut three_index = 0;
        let mut five_index = 0;

        while ugly.len() < n as usize {
            let by2 = ugly[two_index] * 2;
            let by3 = ugly[three_index] * 3;
            let by5 = ugly[five_index] * 5;

            let min = min(by2, min(by3, by5));
            println!("{}", min);
            ugly.push(min);

            if min == by2 {
                two_index += 1;
            }
            if min == by3 {
                three_index += 1;
            }
            if min == by5 {
                five_index += 1;
            }
        }
        ugly.pop().unwrap()
    }
}
// @lc code=end
