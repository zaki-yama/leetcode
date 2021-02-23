/*
 * @lc app=leetcode id=91 lang=rust
 *
 * [91] Decode Ways
 */

// @lc code=start
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.chars().nth(0).unwrap() == '0' {
            return 0;
        }
        let mut dp: Vec<i32> = vec![0; s.len() + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=s.len() {
            let first = s[i - 1..i].parse::<i32>().unwrap();
            let second = s[i - 2..i].parse::<i32>().unwrap();

            if first >= 1 && first <= 9 {
                dp[i] += dp[i - 1];
            }
            if second >= 10 && second <= 26 {
                dp[i] += dp[i - 2];
            }
        }
        dp[s.len()]
    }
}
// @lc code=end
