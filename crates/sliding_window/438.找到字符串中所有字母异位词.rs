/*
 * @lc app=leetcode.cn id=438 lang=rust
 *
 * [438] 找到字符串中所有字母异位词
 */

// @lc code=start

impl crate::Solution {
    pub fn find_anagrams(raw_string: String, template_string: String) -> Vec<i32> {
        let mut template_chars_count =
            template_string
                .bytes()
                .fold([0; 26], |mut template_chars_count, char| {
                    template_chars_count[(char - b'a') as usize] -= 1;
                    template_chars_count
                });
        let raw_chars = raw_string.as_bytes();
        let mut ans = vec![];
        let mut l_index = 0;

        for (r_index, &char) in raw_chars.iter().enumerate() {
            let char = (char - b'a') as usize;
            template_chars_count[char] += 1; // 右端点字母进入窗口

            while template_chars_count[char] > 0 {
                template_chars_count[(raw_chars[l_index] - b'a') as usize] -= 1; // 左端点字母离开窗口
                l_index += 1;
            }
            if r_index - l_index + 1 == template_string.len() {
                // s' 和 p 的每种字母的出现次数都相同
                ans.push(l_index as i32); // s' 左端点下标加入答案
            }
        }

        ans
    }
}

// @lc code=end
