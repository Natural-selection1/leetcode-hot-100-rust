/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

// @lc code=start
impl crate::Solution {
    // 前后缀
    pub fn trap(heights: Vec<i32>) -> i32 {
        let prefix_max: Vec<i32> = heights
            .iter()
            .scan(0, |max, &h| {
                *max = (*max).max(h);
                Some(*max)
            })
            .collect();

        // 这里初始化的的后缀最大值方向是反的，后面迭代的时候需要再反过来
        let postfix_max: Vec<i32> = heights
            .iter()
            .rev()
            .scan(0, |max, &h| {
                *max = (*max).max(h);
                Some(*max)
            })
            .collect();

        heights
            .into_iter()
            .zip(prefix_max)
            .zip(postfix_max.into_iter().rev())
            .map(|((h, prefix), postfix)| prefix.min(postfix) - h)
            .sum()
    }

    // 单调栈
    pub fn trap_(heights: Vec<i32>) -> i32 {
        let mut total_water = 0;
        // 存下标用来计算宽度
        let mut decreasing_stack: Vec<usize> = Vec::new();

        for (right_index, &height) in heights.iter().enumerate() {
            // 弹出所有小于等于当前高度的柱子，计算积水
            while let Some(bottom_index) =
                decreasing_stack.pop_if(|bottom_index| heights[*bottom_index] <= height)
            {
                let bottom_height = heights[bottom_index];
                let Some(&left_index) = decreasing_stack.last() else {
                    continue;
                };
                let left_height = heights[left_index];
                let water_height = left_height.min(height) - bottom_height;
                let water_width = (right_index - left_index - 1) as i32;
                total_water += water_height * water_width;
            }
            decreasing_stack.push(right_index);
        }

        total_water
    }
}
// @lc code=end
