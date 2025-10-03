/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] 滑动窗口最大值
 */

// @lc code=start
use std::collections::VecDeque;

impl crate::Solution {
    pub fn max_sliding_window(num_list: Vec<i32>, window_size: i32) -> Vec<i32> {
        let window_size = window_size as usize;
        let result_len = num_list.len() - window_size + 1;
        let mut result = Vec::with_capacity(result_len);
        // 存放的是元素的下标
        let mut decreasing_deque = VecDeque::with_capacity(window_size + 1);

        for (index, &num) in num_list.iter().enumerate() {
            // * 维持单调递减
            while decreasing_deque
                .back()
                .is_some_and(|&index| num_list[index] <= num)
            {
                decreasing_deque.pop_back();
            }

            decreasing_deque.push_back(index);
            // * 移除离开窗口的元素
            #[allow(clippy::unwrap_used, reason = "窗口内总是至少有一个元素")]
            if index == decreasing_deque.front().unwrap() + window_size {
                decreasing_deque.pop_front();
            }

            // * 只要开始拖动窗口, 就开始记录答案
            if index >= window_size - 1 {
                #[allow(clippy::unwrap_used, reason = "窗口内总是至少有一个元素")]
                result.push(num_list[*decreasing_deque.front().unwrap()]);
            }
        }

        result
    }
}
// @lc code=end
