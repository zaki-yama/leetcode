#[test]
fn test() {
    assert_eq!(2, Solution::num_decodings(String::from("11106")));
    assert_eq!(0, Solution::num_decodings(String::from("0")));
    assert_eq!(2, Solution::num_decodings(String::from("12")));
    assert_eq!(3, Solution::num_decodings(String::from("226")));
}

fn main() {
    // println!("{}", Solution::halves_are_alike(String::from(raw_str)));
}

struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.chars().nth(0).unwrap() == '0' {
            return 0;
        }
        let mut dp: Vec<i32> = vec![0; s.len() + 1];
        println!("{}", dp.len());
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=s.len() {
            let first = s[i - 1..i].parse::<i32>().unwrap();
            let second = s[i - 2..i].parse::<i32>().unwrap();

            println!("first: {}, second: {}", first, second);
            if first >= 1 && first <= 9 {
                dp[i] += dp[i - 1];
            }
            if second >= 10 && second <= 26 {
                dp[i] += dp[i - 2];
            }
            println!("dp: {:?}", dp);
        }
        dp[s.len()]
    }
}
