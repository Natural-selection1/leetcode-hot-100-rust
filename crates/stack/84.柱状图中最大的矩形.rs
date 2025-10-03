/*
 * @lc app=leetcode.cn id=84 lang=rust
 *
 * [84] 柱状图中最大的矩形
 */

// @lc code=start
impl crate::Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        heights.push(0); // 用来在最后把栈清空
        let mut increasing_stack = vec![];
        let mut answer = 0;

        for (r_index, &height) in heights.iter().enumerate() {
            // 当所给数组被多个 0 进行分隔后, 栈底会出现 0 囤积
            // 但是 0 作为高度计算出的面积不可能会是更大的值, 所以忽略是可行的
            while let Some(h_index) = increasing_stack.pop_if(|i| height < heights[*i]) {
                let height = heights[h_index];
                let width = increasing_stack
                    .last()
                    .map(|&l_index| r_index - l_index - 1)
                    .unwrap_or(r_index);

                answer = answer.max(height * width as i32);
            }
            increasing_stack.push(r_index);
        }

        answer
    }
}
// @lc code=end
