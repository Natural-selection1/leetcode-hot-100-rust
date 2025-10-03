/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

use crate::Solution;
// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut left_index = 0;
        let mut right_index = height.len() - 1;

        while let weith = (right_index - left_index) as i32
            && let current_height = height[right_index].min(height[left_index])
            && weith.is_positive()
        {
            max = max.max(weith * current_height);
            match height[right_index] > height[left_index] {
                true => left_index += 1,
                false => right_index -= 1,
            }
        }

        max
    }
}
// @lc code=end
