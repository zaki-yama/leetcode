/*
 * @lc app=leetcode id=821 lang=rust
 *
 * [821] Shortest Distance to a Character
 */

// @lc code=start
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for (i, char) in s.chars().enumerate() {
            if char == c {
                ans.push(0);
                continue;
            }

            // search left
            let mut cur: i32 = i as i32;
            let mut left = std::i32::MAX;
            while cur >= 0 {
                if s.chars().nth(cur as usize).unwrap() == c {
                    left = (i as i32) - (cur as i32);
                    break;
                }
                cur -= 1;
            }

            // search right
            cur = i as i32;
            let mut right = std::i32::MAX;
            while cur < s.len() as i32 {
                if s.chars().nth(cur as usize).unwrap() == c {
                    right = (cur as i32) - (i as i32);
                    break;
                }
                cur += 1;
            }

            ans.push(left.min(right));
        }
        ans
    }
}
// @lc code=end
