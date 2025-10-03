/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
use std::cmp::Ordering;

impl crate::Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let length = nums.len();
        let mut result = vec![];
        let max_two_sum = nums[length - 2] + nums[length - 1];

        for (l_index, &l_val) in nums.iter().enumerate().take(length - 2) {
            // 加上最大的两个数都不超过0, 或者这个数字已经找过一次了
            if max_two_sum + l_val < 0 || nums.get(l_index - 1) == Some(&l_val) {
                continue;
            }
            if l_val.is_positive() {
                break;
            }

            // 把 `三数之和` 简化为 `两数之和`
            let mut m_index = l_index + 1;
            let mut r_index = length - 1;
            while m_index < r_index {
                let m_val = nums[m_index];
                let r_val = nums[r_index];
                let sum = l_val + m_val + r_val;

                match sum.cmp(&0) {
                    Ordering::Less => m_index += 1,
                    Ordering::Greater => r_index -= 1,
                    Ordering::Equal => {
                        // 只有中值与上一个不同时(那么另一个数也肯定不同)
                        // 或者这是 -1, -1, 2 这种情况才记录
                        if m_val != nums[m_index - 1] || m_index == l_index + 1 {
                            result.push(vec![l_val, m_val, r_val]);
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
