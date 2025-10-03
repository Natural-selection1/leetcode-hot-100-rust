/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;
        nums.sort_unstable();
        let mut result = vec![];

        let length = nums.len();
        let biggest_two_num_sum = nums[length - 2] + nums[length - 1];

        for walk_index in 0..length - 2 {
            let walk_val = nums[walk_index];
            if walk_index >= 1 && walk_val == nums[walk_index - 1] {
                continue;
            }
            if walk_val + nums[walk_index + 1] + nums[walk_index + 2] > 0 {
                break;
            }
            if walk_val + biggest_two_num_sum < 0 {
                continue;
            }

            let mut m_index = walk_index + 1;
            let mut r_index = length - 1;
            // * 把 `三数之和` 简化为 `两数之和`
            while m_index < r_index {
                let m_val = nums[m_index];
                let r_val = nums[r_index];
                let sum = walk_val + m_val + r_val;

                match sum.cmp(&0) {
                    Ordering::Greater => r_index -= 1,
                    Ordering::Less => m_index += 1,
                    Ordering::Equal => {
                        if m_index == walk_index + 1 // * 要么是第一次
                        // * 若不是第一次, 但是构成的数和上一次不同 (另一个数也肯定不同)
                        || m_val != nums[m_index - 1]
                        {
                            result.push(vec![walk_val, m_val, r_val]);
                        }
                        m_index += 1;
                        r_index -= 1;
                    }
                }
            }
        }
        result
    }
}
// @lc code=end
