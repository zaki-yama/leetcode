/*
 * @lc app=leetcode id=119 lang=rust
 *
 * [119] Pascal's Triangle II
 */

// @lc code=start
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut triangle: Vec<Vec<i32>> = Vec::new();
        triangle.push(vec![1]);
        for i in 1..=row_index {
            let mut row: Vec<i32> = Vec::new();
            let prev_row = triangle.get((i - 1) as usize);
            for j in 0..=i {
                println!("({}, {})", i, j);
                match prev_row {
                    Some(v) => {
                        let a = v.get((j - 1) as usize).unwrap_or(&0);
                        let b = v.get(j as usize).unwrap_or(&0);
                        println!("({}, {}): {}", i, j, a + b);
                        row.push(a + b);
                    }
                    None => {
                        row.push(0);
                    }
                }
            }
            println!("{:?}", row);
            triangle.push(row);
        }
        println!("{:?}", triangle);
        triangle[(row_index) as usize].clone()
    }
}
// @lc code=end
