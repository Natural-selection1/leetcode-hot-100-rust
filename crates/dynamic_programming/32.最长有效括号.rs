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
            // |reconnected_len|                    |    neighboring_matched_len    |
            // (      ...      )            ?       (              ...              ) )
            //                 nmpi - 2  nmpi - 1   neighboring_matched_pre_index     index

            let neighboring_matched_len = longest_len_when_arrive[index - 1];
            let neighboring_matched_pre_index = index - neighboring_matched_len;

            if let Some('(') = raw_string.get(neighboring_matched_pre_index - 1) {
                let reconnected_len = longest_len_when_arrive
                    .get(neighboring_matched_pre_index - 2)
                    .unwrap_or(&0);

                longest_len_when_arrive[index] = 2 + neighboring_matched_len + reconnected_len;
            }
        }

        longest_len_when_arrive.into_iter().max().unwrap_or(0) as _
    }
}
// @lc code=end
