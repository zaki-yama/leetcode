/*
 * @lc app=leetcode id=821 lang=rust
 *
 * [821] Shortest Distance to a Character
 */

// @lc code=start
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let mut prev = std::i32::MIN / 2;
        for (i, char) in s.chars().enumerate() {
            println!("{}: {}", i, char);
            if char == c {
                prev = i as i32;
            }
            ans.push(i as i32 - prev);
            println!("{}", ans[i]);
        }

        prev = std::i32::MAX / 2;
        for (i, char) in s.chars().rev().enumerate() {
            println!("{}: {}", i, char);
            let j = s.len() - i - 1;
            if char == c {
                prev = j as i32;
            }
            ans[j] = ans[j].min(prev - j as i32);
            println!("{}", ans[j]);
        }
        ans
    }
}
// @lc code=end
