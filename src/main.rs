fn main() {
    println!("{:}", Solution::h_index(vec![0]));
    println!("{:}", Solution::h_index(vec![100]));
    println!("{:}", Solution::h_index(vec![0, 1, 3, 5, 6]));
    println!("{:}", Solution::h_index(vec![0, 1, 4, 5, 6]));
    println!("{:}", Solution::h_index(vec![1, 2]));
}

struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut h_index = 0;
        let mut cnt = 1;
        if citations.len() == 1 {
            if *citations.get(0).unwrap() < 1 {
                return 0;
            } else {
                return 1;
            }
        }

        for citation in citations.into_iter().rev() {
            // println!("----------");
            // println!("h_index: {:}", h_index);
            // println!("cnt: {:} vs citation: {:}", cnt, citation);
            if citation < cnt as i32 {
                break;
            }
            h_index = citation;
            cnt += 1;
        }
        cnt - 1
    }
}
