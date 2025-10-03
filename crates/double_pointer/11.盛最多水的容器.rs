/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
impl crate::Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_capacity = 0;
        let mut l_index = 0;
        let mut r_index = height.len() - 1;

        while let weith = (r_index - l_index) as i32
            && weith.is_positive()
        {
            let current_height = Ord::min(height[l_index], height[r_index]);
            max_capacity = max_capacity.max(weith * current_height);

            match height[r_index] > height[l_index] {
                true => l_index += 1,
                false => r_index -= 1,
            }
        }

        max_capacity
    }
}
// @lc code=end
