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

        for (index, num) in nums.iter().enumerate() {
            let difference = target - num;

            match value_index_map.get(&difference) {
                Some(&first_index) => return vec![first_index as i32, index as i32],
                None => _ = value_index_map.insert(num, index),
            }
        }

        unreachable!()
    }
}
// @lc code=end
