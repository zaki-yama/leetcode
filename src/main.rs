#[test]
fn test() {
    assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
    assert_eq!(104, Solution::rob(vec![1, 2, 3, 1, 1, 100, 4]));
    assert_eq!(0, Solution::rob(vec![]));
    assert_eq!(1, Solution::rob(vec![1, 1]));
    assert_eq!(1, Solution::rob(vec![1]));
}

fn main() {
    // println!("{}", Solution::halves_are_alike(String::from(raw_str)));
}

struct Solution;
use std::cmp;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut dp = vec![0; nums.len() + 1];

        dp[0] = 0;
        dp[1] = nums[0];

        for i in 1..nums.len() {
            println!("[{}] {}", i, nums[i]);
            dp[i + 1] = cmp::max(dp[i - 1] + nums[i], dp[i]);
        }
        println!("dp: {:?}", dp);
        dp[nums.len()]
    }
}
