/*
 * @lc app=leetcode.cn id=739 lang=rust
 *
 * [739] 每日温度
 *
 * https://leetcode.cn/problems/daily-temperatures/description/
 *
 * algorithms
 * Medium (69.79%)
 * Likes:    2197
 * Dislikes: 0
 * Total Accepted:    1M
 * Total Submissions: 1.5M
 * Testcase Example:  '[73,74,75,71,69,72,76,73]'
 *
 * 给定一个整数数组 temperatures ，表示每天的温度，返回一个数组 answer ，其中 answer[i] 是指对于第 i
 * 天，下一个更高温度出现在几天后。如果气温在这之后都不会升高，请在该位置用 0 来代替。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: temperatures = [73,74,75,71,69,72,76,73]
 * 输出: [1,1,4,2,1,1,0,0]
 *
 *
 * 示例 2:
 *
 *
 * 输入: temperatures = [30,40,50,60]
 * 输出: [1,1,1,0]
 *
 *
 * 示例 3:
 *
 *
 * 输入: temperatures = [30,60,90]
 * 输出: [1,1,0]
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= temperatures.length <= 10^5
 * 30 <= temperatures[i] <= 100
 *
 *
 */

// @lc code=start
impl crate::Solution {
    // 倒序遍历
    pub fn daily_temperatures_(temperatures: Vec<i32>) -> Vec<i32> {
        todo!()
    }

    // 正序遍历
    pub fn daily_temperatures(nums: Vec<i32>) -> Vec<i32> {
        // 存放非严格递减的数的下标
        let mut de_stack: Vec<usize> = vec![];
        let mut result = vec![0; nums.len()];

        for r_index in 0..nums.len() {
            let r_num = nums[r_index];

            // 维护非严格单调递减, 并记录答案
            while !de_stack.is_empty()
                && nums[*de_stack.last().unwrap()] < r_num
            {
                let previous_index = de_stack.pop().unwrap();
                result[previous_index] = (r_index - previous_index) as i32;
            }
            de_stack.push(r_index);
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

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let data_len = temperatures.len();
    let mut answer = vec![0; data_len];
    let mut decreasing_stack = vec![];

    for (index, &temperature) in temperatures.iter().enumerate().rev() {
        while matches!(decreasing_stack.last(), Some(&last_index) if temperature >= temperatures[last_index])
        {
            decreasing_stack.pop();
        }

        decreasing_stack
            .last()
            .map(|&exist_index| answer[index] = (exist_index - index) as i32);
        decreasing_stack.push(index);
    }

    answer
}

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

*/
