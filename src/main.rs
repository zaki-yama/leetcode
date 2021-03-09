#[test]
fn test() {
    assert_eq!(3, Solution::num_squares(12));
    assert_eq!(2, Solution::num_squares(13));

    assert_eq!(1, Solution::num_squares(1));
    assert_eq!(2, Solution::num_squares(2));
    assert_eq!(3, Solution::num_squares(3));
    assert_eq!(1, Solution::num_squares(4));
    assert_eq!(2, Solution::num_squares(5));
    assert_eq!(4, Solution::num_squares(7));
    assert_eq!(2, Solution::num_squares(8));
    assert_eq!(1, Solution::num_squares(9));
    assert_eq!(2, Solution::num_squares(10));
    assert_eq!(3, Solution::num_squares(11));
}

fn main() {
    // println!("{}", Solution::halves_are_alike(String::from(raw_str)));
    assert_eq!(3, Solution::num_squares(12));
}

struct Solution;
use std::cmp;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut ans = vec![std::i32::MAX - 1; (n + 1) as usize];
        ans[0] = 0;
        for i in 1..=n {
            println!("i = {}", i);
            let mut j = 1;
            while j * j <= i {
                ans[i as usize] = cmp::min(ans[i as usize], ans[(i - j * j) as usize] + 1);
                j += 1;
            }
        }
        ans[n as usize]
    }
}
