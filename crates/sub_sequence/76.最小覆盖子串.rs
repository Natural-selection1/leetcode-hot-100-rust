/*
 * @lc app=leetcode.cn id=76 lang=rust
 *
 * [76] 最小覆盖子串
 */

// @lc code=start
use std::collections::HashMap;

impl crate::Solution {
    pub fn min_window(raw_string: String, template_string: String) -> String {
        let raw_chars: Vec<char> = raw_string.chars().collect();
        let mut chars_count_map = template_string
            .chars()
            .fold(HashMap::new(), |mut map, char| {
                *map.entry(char).or_insert(0) -= 1;
                map
            });
        let mut l_index = 0;
        let mut answer = (usize::MIN, usize::MAX);

        for (r_index, r_char) in raw_chars.iter().enumerate() {
            // * 仅计数 模版中的字符
            match chars_count_map.get_mut(r_char) {
                Some(count) => *count += 1,
                None => continue,
            }

            while chars_count_map[&raw_chars[l_index]] > 0
                // * 左边界的字符必然在模板中, 否则左边界必然可以向右缩小
                || !chars_count_map.contains_key(&raw_chars[l_index])
            {
                if let Some(x) = chars_count_map.get_mut(&raw_chars[l_index]) {
                    *x -= 1;
                }
                l_index += 1;
            }

            if r_index - l_index < answer.1 - answer.0
                && chars_count_map.values().all(|count| *count >= 0)
                // 至少有一个为刚好满足时, 才有可能最少
                && chars_count_map.values().any(|count| *count == 0)
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
