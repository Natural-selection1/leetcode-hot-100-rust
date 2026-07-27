/*
 * @lc app=leetcode.cn id=32 lang=rust
 *
 * [32] 最长有效括号
 */

// @lc code=start
impl crate::Solution {
    pub fn longest_valid_parentheses(raw_string: String) -> i32 {
        let raw_string: Vec<char> = raw_string.chars().collect();
        let mut longest_len_when_arrive = vec![0; raw_string.len()];

        for (index, _) in raw_string
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(_, char)| char == &&')')
        {
            // |reconnected_len|                    |      adjacent_matched_len     |
            // (      ...      )            ?       (              ...              ) )
            //                 ampi - 2  ampi - 1   adjacent_matched_pre_index        index

            let adjacent_matched_len = longest_len_when_arrive[index - 1];
            let adjacent_matched_pre_index = index - adjacent_matched_len;

            if let Some('(') = raw_string.get(adjacent_matched_pre_index - 1) {
                let reconnected_len = longest_len_when_arrive
                    .get(adjacent_matched_pre_index - 2)
                    .unwrap_or(&0);

                longest_len_when_arrive[index] = reconnected_len + 2 + adjacent_matched_len;
            }
        }

        longest_len_when_arrive.into_iter().max().unwrap_or(0) as _
    }
}
// @lc code=end
