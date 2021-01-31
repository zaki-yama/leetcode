/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = std::i32::MAX;
        let mut max_profit = 0;

        for price in prices.iter() {
            if *price < min_price {
                min_price = *price;
            } else if *price - min_price > max_profit {
                max_profit = *price - min_price;
            }
        }
        max_profit
    }
}
// @lc code=end
