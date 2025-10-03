/*
 * @lc app=leetcode.cn id=72 lang=rust
 *
 * [72] 编辑距离
 */

// @lc code=start
impl crate::Solution {
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
                    #[allow(clippy::unwrap_used, reason = "所给切片不为空")]
                    false => dp[col + 1] = [prev, dp[col + 1], dp[col]].iter().min().unwrap() + 1,
                }
                prev = unchanged_cur
            }
        }

        dp[text2_len] as i32
    }
}
// @lc code=end
