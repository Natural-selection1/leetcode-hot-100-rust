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

        for (right_index, &right_height) in heights.iter().enumerate() {
            loop {
                if matches!(decreasing_stack.last(),
                    Some(&last_index) if right_height >= heights[last_index])
                {
                    let bottom_height = {
                        let bottom_index = decreasing_stack.pop().unwrap();
                        heights[bottom_index]
                    };

                    let left_index = match decreasing_stack.last() {
                        Some(index) => *index,
                        None => continue,
                    };

                    let left_height = heights[left_index];
                    total_water += {
                        let water_height = left_height.min(right_height) - bottom_height;
                        let water_width = (right_index - left_index - 1) as i32;
                        water_height * water_width
                    };
                } else {
                    break;
                }
            }
            decreasing_stack.push(right_index);
        }

        total_water
    }
}
// @lc code=end
