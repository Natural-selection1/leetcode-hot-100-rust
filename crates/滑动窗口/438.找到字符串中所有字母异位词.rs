/*
 * @lc app=leetcode.cn id=438 lang=rust
 *
 * [438] 找到字符串中所有字母异位词
 */

// @lc code=start
use std::collections::HashMap;

impl crate::Solution {
    pub fn find_anagrams(raw_string: String, template_string: String) -> Vec<i32> {
        let raw_chars: Vec<char> = raw_string.chars().collect();
        let template_len = template_string.len();
        let mut char_count_map: HashMap<char, i32> =
            template_string
                .chars()
                .fold(HashMap::new(), |mut map, char| {
                    *map.entry(char).or_insert(0) -= 1;
                    map
                });
        let mut ans = vec![];
        let mut l_index = 0;

        for (r_index, r_char) in raw_chars.iter().enumerate() {
            *char_count_map.entry(*r_char).or_insert(0) += 1;

            while let Some(&count) = char_count_map.get(r_char) {
                if count <= 0 {
                    break;
                }
                let l_char = raw_chars[l_index];
                char_count_map.entry(l_char).and_modify(|count| *count -= 1);
                l_index += 1;
            }

            if r_index - l_index + 1 == template_len {
                ans.push(l_index as i32);
            }
        }

        ans
    }
}
// @lc code=end
