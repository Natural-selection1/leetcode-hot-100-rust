/*
 * @lc app=leetcode.cn id=76 lang=rust
 *
 * [76] 最小覆盖子串
 */

// @lc code=start
#![allow(clippy::option_map_unit_fn)]
use std::collections::HashMap;

impl crate::Solution {
    pub fn min_window(raw_string: String, template_string: String) -> String {
        if template_string.len() > raw_string.len() {
            return String::new();
        }

        let raw_chars = raw_string.as_bytes();
        let mut chars_count_map =
            template_string
                .as_bytes()
                .iter()
                .fold(HashMap::new(), |mut map, &ch| {
                    map.entry(ch).and_modify(|count| *count -= 1).or_insert(-1);
                    map
                });
        let mut l_index = 0;
        let mut answer = (usize::MIN, usize::MAX);

        for (r_index, r_char) in raw_chars.iter().enumerate() {
            match chars_count_map.get_mut(r_char) {
                Some(count) => *count += 1,
                None => continue,
            }

            while let l_char = raw_chars[l_index]
                // 左边界的字符肯定在模板中, 否则窗口必然可以继续缩小
                && (!chars_count_map.contains_key(&l_char) || chars_count_map[&l_char] > 0)
            {
                chars_count_map.get_mut(&l_char).map(|count| *count -= 1);
                l_index += 1;
            }

            // 必然全都字符数目都满足, 且至少有一个刚好满足(才会是最少)
            if r_index - l_index < answer.1 - answer.0
                && chars_count_map.values().any(|count| count == &0)
                && chars_count_map.values().all(|count| count >= &0)
            {
                answer = (l_index, r_index);
            }
        }

        match answer.1 == usize::MAX {
            true => String::new(),
            false => raw_string[answer.0..=answer.1].to_string(),
        }
    }
}
// @lc code=end
