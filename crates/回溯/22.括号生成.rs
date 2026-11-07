/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 *
 * https://leetcode.cn/problems/generate-parentheses/description/
 *
 * algorithms
 * Medium (79.10%)
 * Likes:    4131
 * Dislikes: 0
 * Total Accepted:    1.3M
 * Total Submissions: 1.7M
 * Testcase Example:  '3'
 *
 * 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 3
 * 输出：["((()))","(()())","(())()","()(())","()()()"]
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 1
 * 输出：["()"]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n <= 8
 *
 *
 */

// @lc code=start
impl crate::Solution {
    pub fn generate_parenthesis(size: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut path: String = String::new();

        backtracing(size, &mut result, &mut path, size, size);

        result
    }
}

fn backtracing(
    size: i32,
    result: &mut Vec<String>,
    path: &mut String,
    unused_left: i32,
    unused_right: i32,
) {
    if path.len() == (size *2) as usize {
        result.push(path.clone());
        return;
    }

    if unused_left > 0 {
        path.push('(');
        backtracing(size, result, path, unused_left - 1, unused_right);
        path.pop();
    }

    if unused_right > unused_left {
        path.push(')');
        backtracing(size, result, path, unused_left, unused_right - 1);
        path.pop();
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

pub fn generate_parenthesis(size: i32) -> Vec<String> {
    let mut result = vec![];
    let size = size as usize;
    backtracing(
        size,
        0,
        0,
        &mut String::with_capacity(2 * size),
        &mut result,
    );
    result
}

fn backtracing(
    size: usize,
    used_left: usize,
    used_right: usize,
    temp_answer: &mut String,
    answer: &mut Vec<String>,
) {
    if temp_answer.len() == size * 2 {
        return answer.push(temp_answer.clone());
    }

    if used_left < size {
        temp_answer.push('(');
        backtracing(size, used_left + 1, used_right, temp_answer, answer);
        temp_answer.pop();
    }

    if used_right < used_left {
        temp_answer.push(')');
        backtracing(size, used_left, used_right + 1, temp_answer, answer);
        temp_answer.pop();
    }
}

*/
