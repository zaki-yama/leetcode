fn main() {
    assert_eq!(true, Solution::is_power_of_four(16));
    assert_eq!(false, Solution::is_power_of_four(8));
    assert_eq!(false, Solution::is_power_of_four(5));
    assert_eq!(true, Solution::is_power_of_four(4));
    assert_eq!(false, Solution::is_power_of_four(3));
    assert_eq!(false, Solution::is_power_of_four(2));
    assert_eq!(true, Solution::is_power_of_four(1));
    assert_eq!(false, Solution::is_power_of_four(0));
    assert_eq!(true, Solution::is_power_of_four(268_435_456));
    assert_eq!(false, Solution::is_power_of_four(17));
}

struct Solution;

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
