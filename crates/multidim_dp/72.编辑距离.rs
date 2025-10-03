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
        let mut dcol: Vec<usize> = (0..=text2_len).collect();

        for (row_index, source_char) in text1.chars().enumerate() {
            dcol[0] = row_index + 1; // 行首初始化
            let mut prev = row_index;

            for (col_index, target_char) in text2.chars().enumerate() {
                let unchanged_cur = dcol[col_index + 1];
                match source_char == target_char {
                    true => dcol[col_index + 1] = prev,
                    false => dcol[col_index + 1] = prev.min(unchanged_cur).min(dcol[col_index]) + 1,
                }
                prev = unchanged_cur
            }
        }

        dcol[text2_len] as i32
    }
}
// @lc code=end
