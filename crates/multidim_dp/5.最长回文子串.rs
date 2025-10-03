/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
*/

// @lc code=start
impl crate::Solution {
    pub fn longest_palindrome(raw_string: String) -> String {
        let string_len = raw_string.len();
        let char_of = raw_string.as_bytes();

        let mut is_palindrome_form = vec![vec![true; string_len]; string_len];
        let mut res = (0, 0); // 闭区间

        for len in 1..string_len {
            for l_index in 0..(string_len - len) {
                let r_index = l_index + len;
                let is_inner_palindrome = is_palindrome_form[l_index + 1][r_index - 1];
                let current_is_palindrome = &mut is_palindrome_form[l_index][r_index];
                let is_boundary_char_same = char_of[l_index] == char_of[r_index];

                if len == 1 {
                    *current_is_palindrome = is_boundary_char_same
                } else {
                    *current_is_palindrome = is_boundary_char_same && is_inner_palindrome
                }

                if *current_is_palindrome {
                    res = (l_index, r_index);
                }
            }
        }

        raw_string[res.0..=res.1].to_string()
    }

    pub fn center_expand_longest_palindrome(raw_string: String) -> String {
        let char_of = raw_string.as_bytes();
        let string_len = raw_string.len();

        if string_len <= 1 {
            return raw_string;
        }

        let mut longest_range = (0, 0); // 闭区间
        for i in 0..string_len {
            // 对于奇数长度的回文串
            let searched_range = center_expand(char_of, i, i);
            if searched_range.1 - searched_range.0 > longest_range.1 - longest_range.0 {
                longest_range = searched_range
            }

            // 对于偶数长度的回文串
            if i + 1 < string_len && char_of[i] == char_of[i + 1] {
                let searched_range = center_expand(char_of, i, i + 1);
                if searched_range.1 - searched_range.0 > longest_range.1 - longest_range.0 {
                    longest_range = searched_range
                }
            }
        }
        raw_string[longest_range.0..=longest_range.1].to_string()
    }
}

fn center_expand(char_of: &[u8], mut l: usize, mut r: usize) -> (usize, usize) {
    while l >= 1 && r < char_of.len() - 1 && char_of[l - 1] == char_of[r + 1] {
        l -= 1;
        r += 1;
    }
    (l, r)
}
// @lc code=end
