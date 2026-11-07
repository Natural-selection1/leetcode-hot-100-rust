/*
 * @lc app=leetcode.cn id=72 lang=rust
 *
 * [72] 编辑距离
 *
 * https://leetcode.cn/problems/edit-distance/description/
 *
 * algorithms
 * Medium (64.07%)
 * Likes:    3970
 * Dislikes: 0
 * Total Accepted:    932.6K
 * Total Submissions: 1.5M
 * Testcase Example:  '"horse"\n"ros"'
 *
 * 给你两个单词 word1 和 word2， 请返回将 word1 转换成 word2 所使用的最少操作数  。
 *
 * 你可以对一个单词进行如下三种操作：
 *
 *
 * 插入一个字符
 * 删除一个字符
 * 替换一个字符
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：word1 = "horse", word2 = "ros"
 * 输出：3
 * 解释：
 * horse -> rorse (将 'h' 替换为 'r')
 * rorse -> rose (删除 'r')
 * rose -> ros (删除 'e')
 *
 *
 * 示例 2：
 *
 *
 * 输入：word1 = "intention", word2 = "execution"
 * 输出：5
 * 解释：
 * intention -> inention (删除 't')
 * inention -> enention (将 'i' 替换为 'e')
 * enention -> exention (将 'n' 替换为 'x')
 * exention -> exection (将 'n' 替换为 'c')
 * exection -> execution (插入 'u')
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0 <= word1.length, word2.length <= 500
 * word1 和 word2 由小写英文字母组成
 *
 *
 */

// @lc code=start
impl crate::Solution {
    pub fn min_distance(text1: String, text2: String) -> i32 {
        todo!()
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

pub fn min_distance(text1: String, text2: String) -> i32 {
    if text1.is_empty() || text2.is_empty() {
        return (text1.len() + text2.len()) as i32;
    }

    let text2_len = text2.len();
    // 将空串转换为 text2[0..j] 所需的最小操作数
    let mut dp: Vec<usize> = (0..=text2_len).collect();

    for (row, source_char) in text1.chars().enumerate() {
        dp[0] = row + 1; // 行首初始化
        let mut prev = row;

        for (col, target_char) in text2.chars().enumerate() {
            let unchanged_cur = dp[col + 1];

            match source_char == target_char {
                true => dp[col + 1] = prev,
                false => dp[col + 1] = [prev, dp[col + 1], dp[col]].iter().min().unwrap() + 1,
            }
            prev = unchanged_cur
        }
    }

    dp[text2_len] as i32
}

*/
