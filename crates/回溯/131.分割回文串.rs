/*
 * @lc app=leetcode.cn id=131 lang=rust
 *
 * [131] 分割回文串
 *
 * https://leetcode.cn/problems/palindrome-partitioning/description/
 *
 * algorithms
 * Medium (75.24%)
 * Likes:    2244
 * Dislikes: 0
 * Total Accepted:    825.8K
 * Total Submissions: 1.1M
 * Testcase Example:  '"aab"'
 *
 * 给你一个字符串 s，请你将 s 分割成一些 子串，使每个子串都是 回文串 。返回 s 所有可能的分割方案。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "aab"
 * 输出：[["a","a","b"],["aa","b"]]
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "a"
 * 输出：[["a"]]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length <= 16
 * s 仅由小写英文字母组成
 *
 *
 */

// @lc code=start
impl crate::Solution {
    pub fn partition(raw_string: String) -> Vec<Vec<String>> {
        todo!()
    }
}

fn backtracking(
    raw_string: &str,
    index: usize,
    path: &mut Vec<String>,
    answer: &mut Vec<Vec<String>>,
) {
    todo!()
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

pub fn partition(raw_string: String) -> Vec<Vec<String>> {
    let mut answer = vec![];
    backtracking(&raw_string, 0, &mut vec![], &mut answer);
    answer
}

fn backtracking(
    raw_string: &str,
    start: usize,
    path: &mut Vec<String>,
    answer: &mut Vec<Vec<String>>,
) {
    if start == raw_string.len() {
        return answer.push(path.clone());
    }

    for r_index in start..raw_string.len() {
        let str = &raw_string[start..=r_index];

        if Iterator::eq(str.chars(), str.chars().rev()) {
            path.push(str.to_string());
            backtracking(raw_string, r_index + 1, path, answer);
            path.pop();
        }
    }
}

*/
