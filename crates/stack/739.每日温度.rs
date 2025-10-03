/*
 * @lc app=leetcode.cn id=739 lang=rust
 *
 * [739] 每日温度
 */

// @lc code=start
#![allow(clippy::option_map_unit_fn)]

impl crate::Solution {
    // 倒序遍历
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let data_len = temperatures.len();
        let mut answer = vec![0; data_len];
        let mut decreasing_stack = vec![];

        for (index, &temperature) in temperatures.iter().enumerate().rev() {
            while decreasing_stack
                .pop_if(|last_index| temperature >= temperatures[*last_index])
                .is_some()
            {}

            decreasing_stack
                .last()
                .map(|&exist_index| answer[index] = (exist_index - index) as i32);
            decreasing_stack.push(index);
        }

        answer
    }

    // 正序遍历
    pub fn daily_temperatures_(temperatures: Vec<i32>) -> Vec<i32> {
        let data_len = temperatures.len();
        let mut answer = vec![0; data_len];
        let mut decreasing_stack = vec![];

        for (index, &temperature) in temperatures.iter().enumerate() {
            while let Some(&top_index) = decreasing_stack.last() {
                match temperature > temperatures[top_index] {
                    true => {
                        answer[top_index] = (index - top_index) as i32;
                        decreasing_stack.pop();
                    }
                    false => break,
                }
            }

            decreasing_stack.push(index);
        }

        answer
    }
}
// @lc code=end
