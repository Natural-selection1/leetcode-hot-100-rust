/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] 滑动窗口最大值
 */

// @lc code=start
impl crate::Solution {
    pub fn max_sliding_window(num_list: Vec<i32>, window_size: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        let window_size = window_size as usize;
        let result_len = num_list.len() - window_size + 1;
        let mut result = Vec::with_capacity(result_len);
        let mut index_deque = VecDeque::with_capacity(window_size + 1);

        for (r_index, &r_num) in num_list.iter().enumerate() {
            // * 只要有小于 r_num 的, 全部移除
            while index_deque
                .back()
                .is_some_and(|&index| r_num >= num_list[index])
            {
                index_deque.pop_back();
            }

            index_deque.push_back(r_index);

            // * 移除离开窗口的元素
            if r_index == index_deque.front().unwrap() + window_size {
                index_deque.pop_front();
            }

            // * 只要开始拖动窗口, 就开始记录答案
            if r_index >= window_size - 1 {
                result.push(num_list[*index_deque.front().unwrap()]);
            }
        }

        result
    }
}
// @lc code=end
