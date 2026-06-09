/*
* @lc app=leetcode.cn id=1 lang=rust
*
* [1] 两数之和
*/

// @lc code=start
use std::collections::HashMap;

impl crate::Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut value_index_map = HashMap::new();

        for (index, num) in nums.into_iter().enumerate() {
            let difference = target - num;

            match value_index_map.get(&difference) {
                Some(first_index) => return vec![*first_index as i32, index as i32],
                None => {
                    value_index_map.insert(num, index);
                }
            }
        }

        unreachable!()
    }
}
// @lc code=end

#[test]
fn acm_mode_main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let target: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let result = crate::Solution::two_sum(nums, target);
    println!("{:?}", result);
}
