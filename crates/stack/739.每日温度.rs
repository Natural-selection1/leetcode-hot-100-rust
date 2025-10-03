/*
 * @lc app=leetcode.cn id=739 lang=rust
 *
 * [739] 每日温度
 */

// @lc code=start
impl crate::Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let data_len = temperatures.len();
        let mut answer = vec![0; data_len];
        let mut stack = vec![];

        for (walk_index, &current_temperature) in temperatures.iter().enumerate().rev() {
            while let Some(last_index) = stack.last()
                && current_temperature >= temperatures[*last_index]
            {
                stack.pop();
            }

            if let Some(exist_index) = stack.last() {
                answer[walk_index] = (exist_index - walk_index) as i32;
            }

            stack.push(walk_index);
        }
        answer
    }
}
// @lc code=end
