/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] 滑动窗口最大值
 */

// @lc code=start

impl crate::Solution {
    pub fn max_sliding_window(num_list: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        let k = k as usize;
        // 存的是下标
        let mut de_deque = VecDeque::new();
        let mut result = vec![];
        let mut l_index = 0;

        for r_index in 0..num_list.len() {
            let r_num = num_list[r_index];
            // 维护单调递减
            while !de_deque.is_empty()
                && num_list[*de_deque.back().unwrap()] <= r_num
            {
                de_deque.pop_back();
            }
            de_deque.push_back(r_index);

            // 移除过期元素
            if de_deque.front().unwrap() + k == r_index {
                de_deque.pop_front();
            }

            if r_index + 1 >= k  {
                result.push(num_list[de_deque[0]])
            }
        }

        result
    }
}
// @lc code=end

/*
1
2
3
4
5
6
7
8
9
0
1
2
3
4
5
6
7
8
9
0

pub fn max_sliding_window(num_list: Vec<i32>, window_size: i32) -> Vec<i32> {
    let window_size = window_size as usize;
    let mut result = Vec::with_capacity(num_list.len() - window_size + 1);
    // 存放的是元素的下标
    let mut decreasing_deque = VecDeque::with_capacity(window_size + 1);

    for (index, &num) in num_list.iter().enumerate() {
        // * 维持单调递减
        while matches!(decreasing_deque.back(), Some(&last_index) if num_list[last_index] <= num)
        {
            decreasing_deque.pop_back();
        }

        decreasing_deque.push_back(index);
        // * 移除离开窗口的元素
        if index == decreasing_deque.front().unwrap() + window_size {
            decreasing_deque.pop_front();
        }
        // * 只要开始拖动窗口, 就开始记录答案
        if index + 1 >= window_size {
            result.push(num_list[*decreasing_deque.front().unwrap()]);
        }
    }

    result
}

*/
