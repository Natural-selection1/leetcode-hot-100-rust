/*
* @lc app=leetcode.cn id=1 lang=rust
*
* [1] 两数之和
*/
use crate::Solution;
// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::BTreeMap;
        let mut value_index_map = BTreeMap::new();
        for (second_index, num) in nums.iter().enumerate() {
            let possible_num = target - num;
            let second_index = second_index as i32;
            match value_index_map.get_key_value(&possible_num) {
                Some((_, &first_index)) => return vec![first_index, second_index],
                None => {
                    value_index_map.insert(num, second_index);
                }
            }
        }
        unreachable!()
    }
}
// @lc code=end
