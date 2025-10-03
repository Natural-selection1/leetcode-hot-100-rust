/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
*/

// @lc code=start
impl crate::Solution {
    // 动态规划
    pub fn longest_palindrome_(raw_string: String) -> String {
        let raw_chars = raw_string.as_bytes();
        let string_len = raw_string.len();

        let mut is_palindrome = vec![vec![true; string_len]; string_len];
        let mut longest_range = (0, 0); // 闭区间

        for len in 1..string_len {
            for l in 0..(string_len - len) {
                let r = l + len;
                let is_inner_palindrome = is_palindrome[l + 1][r - 1];
                let is_current_palindrome = &mut is_palindrome[l][r];
                let is_boundary_char_same = raw_chars[l] == raw_chars[r];

                match len == 1 {
                    true => *is_current_palindrome = is_boundary_char_same,
                    false => *is_current_palindrome = is_boundary_char_same && is_inner_palindrome,
                }

                if *is_current_palindrome {
                    longest_range = (l, r);
                }
            }
        }

        raw_string[longest_range.0..=longest_range.1].to_string()
    }

    // 中心扩展法
    pub fn longest_palindrome(raw_string: String) -> String {
        let raw_chars = raw_string.as_bytes();
        let mut range = (0, 0); // 闭区间

        for (i, char) in raw_chars.iter().enumerate() {
            // 分别处理奇偶长度回文串
            update_range(&mut range, center_expand(raw_chars, i, i));
            if Some(char) == raw_chars.get(i + 1) {
                update_range(&mut range, center_expand(raw_chars, i, i + 1));
            }
        }

        raw_string[range.0..=range.1].to_string()
    }
}

fn update_range(range: &mut (usize, usize), searched_range: (usize, usize)) {
    if searched_range.1 - searched_range.0 > range.1 - range.0 {
        *range = searched_range;
    }
}

fn center_expand(raw_chars: &[u8], mut l: usize, mut r: usize) -> (usize, usize) {
    while raw_chars
        .get(l - 1)
        .is_some_and(|l_char| Some(l_char) == raw_chars.get(r + 1))
    {
        l -= 1;
        r += 1;
    }
    (l, r)
}
// @lc code=end
