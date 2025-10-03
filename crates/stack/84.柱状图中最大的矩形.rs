/*
 * @lc app=leetcode.cn id=84 lang=rust
 *
 * [84] 柱状图中最大的矩形
 */

// @lc code=start
impl crate::Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        heights.push(-1); // 用 -1 在未来把栈清空
        let mut stack = vec![-1];
        let mut answer = 0;

        for (r_index, &current_height) in heights.iter().enumerate().map(|(x, y)| (x as i32, y)) {
            while let Some(&top_index) = stack.last()
                && top_index != -1 // 不是哨兵
                && current_height <= heights[top_index as usize]
            {
                let height_index = stack.pop().unwrap() as usize;
                let l_index = *stack.last().unwrap(); // 栈顶下面那个数就是 left

                answer = answer.max(heights[height_index] * (r_index - l_index - 1));
            }
            stack.push(r_index)
        }

        answer
    }
}
// @lc code=end
