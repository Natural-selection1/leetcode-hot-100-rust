/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 */

// @lc code=start
impl crate::Solution {
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
// @lc code=end
