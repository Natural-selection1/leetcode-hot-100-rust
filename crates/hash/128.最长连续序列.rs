/*
 * @lc app=leetcode.cn id=128 lang=rust
 *
 * [128] 最长连续序列
 */

// @lc code=start
use std::collections::HashSet;

impl crate::Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums);
        let half_nums_len = set.len().midpoint(0) as i32;
        let mut longest_sequence_len = 0;

        for &num in &set {
            // * 找到某一个序列的起点
            if set.contains(&(num - 1)) {
                continue;
            }
            // * 当位于某序列的起点, 检查、直到元素不存在
            let current_sequence_length = (num..).take_while(|&x| set.contains(&x)).count();
            longest_sequence_len = longest_sequence_len.max(current_sequence_length as i32);

            if longest_sequence_len >= half_nums_len {
                break;
            }
        }

        longest_sequence_len
    }
}
// @lc code=end
