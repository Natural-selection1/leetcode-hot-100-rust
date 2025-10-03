/*
 * @lc app=leetcode.cn id=438 lang=rust
 *
 * [438] 找到字符串中所有字母异位词
 */

// @lc code=start
impl crate::Solution {
    pub fn find_anagrams(row_string: String, target_string: String) -> Vec<i32> {
        use std::collections::HashMap;
        let row_string_iter: Vec<char> = row_string.chars().collect();
        let target_string_iter: Vec<char> = target_string.chars().collect();
        let mut result = vec![];
        let target_map = {
            let target_string_iter = &target_string_iter;
            let mut char_to_count_map = HashMap::new();
            for char_ascii_num in target_string_iter.iter() {
                *char_to_count_map.entry(char_ascii_num).or_insert(0) += 1;
            }
            char_to_count_map
        };

        let mut l_index = 0;
        let mut walk_map = HashMap::new();

        for (r_index, walk_char) in row_string_iter.iter().enumerate() {
            if !target_map.contains_key(walk_char) {
                for number_count in walk_map.values_mut() {
                    *number_count = 0
                }
                l_index = r_index + 1;
                continue;
            }

            *walk_map.entry(walk_char).or_insert(0) += 1;

            // * target char but count more than requirement
            let target_count = target_map.get(walk_char);
            while walk_map.get(walk_char) > target_count {
                walk_map
                    .entry(&row_string_iter[l_index])
                    .and_modify(|count| *count -= 1);
                l_index += 1;
            }

            (walk_map == target_map).then(|| result.push(l_index as i32));
        }

        result
    }
}
// @lc code=end
