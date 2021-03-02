#[test]
fn test() {
    assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
}

fn main() {
    // println!("{}", Solution::halves_are_alike(String::from(raw_str)));
}

struct Solution;
use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = std::i32::MAX;

        let mut dp = vec![0; prices.len() + 1];
        dp[0] = 0;
        for i in 1..=prices.len() {
            let price = prices[i - 1];
            println!("[{}] {}", i, price);
            if price < min_price {
                min_price = prices[i - 1];
            }
            dp[i] = cmp::max(dp[i - 1], price - min_price);
        }
        dp[prices.len()]
    }
}
