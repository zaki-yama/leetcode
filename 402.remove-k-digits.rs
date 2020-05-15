/*
 * @lc app=leetcode id=402 lang=rust
 *
 * [402] Remove K Digits
 */

// @lc code=start
impl Solution {
  pub fn remove_kdigits(num: String, k: i32) -> String {
    println!("num: {:?}", num);
    let mut stack: Vec<char> = Vec::new();
    let mut k2 = k;

    let mut chars = num.chars();
    loop {
      let next = chars.next();
      if next == None {
        break;
      }

      let digit_char = next.unwrap();
      let digit = digit_char.to_digit(10).unwrap();
      while stack.len() > 0 && k2 > 0 && stack[stack.len() - 1].to_digit(10).unwrap() > digit {
        stack.pop();
        k2 -= 1;
      }

      stack.push(digit_char);
    }

    println!("loop end: {:?} {}", stack, k2);
    while k2 > 0 {
      stack.pop();
      k2 -= 1;
    }

    if stack.len() == 0 {
      return String::from("0");
    }
    println!("before: {:?}", stack);
    while stack.len() > 1 && stack[0] == '0' {
      stack.remove(0);
    }
    println!("after: {:?}", stack);

    stack.into_iter().collect()
  }
}
// @lc code=end
