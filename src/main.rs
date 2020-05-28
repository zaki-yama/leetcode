fn main() {
    println!("{:?}", Solution::count_bits(5));
}

struct Solution;

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut x: Vec<i32> = Vec::with_capacity((num as usize) + 1);
        for i in 0..num + 1 {
            x.push(i.count_ones() as i32);
        }

        x
    }
}
