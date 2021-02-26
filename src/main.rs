#[test]
fn test() {
    assert_eq!(5, Solution::num_trees(3));
    assert_eq!(1, Solution::num_trees(1));
    // ref. https://en.wikipedia.org/wiki/Catalan_number
    assert_eq!(14, Solution::num_trees(4));
    assert_eq!(42, Solution::num_trees(5));
    assert_eq!(16796, Solution::num_trees(10));
}

fn main() {
    // println!("{}", Solution::halves_are_alike(String::from(raw_str)));
}

struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;

        for k in 2..=n {
            for i in 0..k {
                dp[k as usize] += dp[i as usize] * dp[(k - i - 1) as usize];
            }
        }
        dp[n as usize]
    }
}
