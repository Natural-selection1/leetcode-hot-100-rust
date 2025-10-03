/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 */

// @lc code=start
impl crate::Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        back_trace(
            &mut String::with_capacity((2 * n) as usize),
            n as usize,
            0,
            0,
            &mut result,
        );
        result
    }
}
fn back_trace(
    current_vec: &mut String,
    target: usize,
    used_left: usize,
    used_right: usize,
    answer: &mut Vec<String>,
) {
    if current_vec.len() == target * 2 {
        return answer.push(current_vec.clone());
    }

    if used_left < target {
        current_vec.push('(');
        back_trace(current_vec, target, used_left + 1, used_right, answer);
        current_vec.pop();
    }

    if used_right < used_left {
        current_vec.push(')');
        back_trace(current_vec, target, used_left, used_right + 1, answer);
        current_vec.pop();
    }
}
// @lc code=end
