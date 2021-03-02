/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = std::i32::MAX;

        let mut dp = vec![0; prices.len() + 1];
        dp[0] = 0;
        for i in 1..=prices.len() {
            let price = prices[i - 1];
            if price < min_price {
                min_price = prices[i - 1];
            }
            dp[i] = cmp::max(dp[i - 1], price - min_price);
        }
        dp[prices.len()]
    }
}
// @lc code=end
