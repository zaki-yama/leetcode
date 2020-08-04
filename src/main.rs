fn main() {
    assert_eq!(2, Solution::climb_stairs(2));
    assert_eq!(2, Solution::climb_stairs(2));
    assert_eq!(3, Solution::climb_stairs(3));
    assert_eq!(5, Solution::climb_stairs(4));
    assert_eq!(1_134_903_170, Solution::climb_stairs(44));
    assert_eq!(1, fibonacci(1));
    assert_eq!(1, fibonacci(2));
    assert_eq!(2, fibonacci(3));
    assert_eq!(3, fibonacci(4));
    assert_eq!(5, fibonacci(5));
    assert_eq!(8, fibonacci(6));
    // assert_eq!(130, fibonacci(7));
}

fn fibonacci(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
struct Solution;

// impl Solution {
//     pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
//         let word_vec: Vec<char> = word.as_str().chars().collect();
//         for row in board {
//             println!("{:?}", row);
//             for cell in row {
//                 if cell == word_vec[0] {
//                     println!("found {:}", cell);
//                 }
//             }
//         }
//         true
//     }
// }

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut dp = (1, 2);
        for k in 3..=n {
            dp = (dp.1, dp.0 + dp.1);
        }
        dp.1
    }
}
