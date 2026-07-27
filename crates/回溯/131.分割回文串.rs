/*
 * @lc app=leetcode.cn id=131 lang=rust
 *
 * [131] 分割回文串
 */

// @lc code=start
impl crate::Solution {
    pub fn partition(raw_string: String) -> Vec<Vec<String>> {
        let mut answer = vec![];
        backtracking(&raw_string, 0, &mut vec![], &mut answer);
        answer
    }
}

fn backtracking(
    raw_string: &str,
    index: usize,
    path: &mut Vec<String>,
    answer: &mut Vec<Vec<String>>,
) {
    if index == raw_string.len() {
        return answer.push(path.clone());
    }

    for walk_index in index..raw_string.len() {
        let str = &raw_string[index..=walk_index];

        if Iterator::eq(str.chars(), str.chars().rev()) {
            path.push(str.to_string());
            backtracking(raw_string, walk_index + 1, path, answer);
            path.pop();
        }
    }
}
// @lc code=end
