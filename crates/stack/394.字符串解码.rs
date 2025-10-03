/*
 * @lc app=leetcode.cn id=394 lang=rust
 *
 * [394] 字符串解码
 */

// @lc code=start
impl crate::Solution {
    // 栈模拟递归
    pub fn decode_string(raw_string: String) -> String {
        let mut stack = vec![];
        let mut string_buf = vec![];
        let mut repeat_times = 0;

        for char in raw_string.as_bytes() {
            match char {
                b'0'..=b'9' => repeat_times = repeat_times * 10 + (char - b'0') as usize,
                b'[' => {
                    stack.push((repeat_times, std::mem::take(&mut string_buf)));
                    repeat_times = 0;
                }
                b']' => {
                    let Some((repeat_times, mut exist_buf)) = stack.pop() else {
                        unreachable!()
                    };
                    exist_buf.extend_from_slice(&string_buf.repeat(repeat_times));
                    string_buf = exist_buf;
                }
                _ => string_buf.push(*char),
            }
        }

        // !SAFE: 输入保证是合法的 ASCII 字符串
        unsafe { String::from_utf8_unchecked(string_buf) }
    }

    //  递归写法
    pub fn decode_string_(raw_string: String) -> String {
        let raw_string = raw_string.as_bytes();
        let mut cursor = 0;
        decode(raw_string, &mut cursor)
    }
}

fn decode(row_string: &[u8], cursor: &mut usize) -> String {
    let mut formatted_string = String::new();
    let mut repeat_times = 0;

    while *cursor < row_string.len() {
        let char = row_string[*cursor];
        *cursor += 1;

        match char {
            b'0'..=b'9' => repeat_times = (repeat_times * 10) + (char - b'0'),
            b'[' => {
                let sub_formatted_string = decode(row_string, cursor);
                formatted_string.push_str(&sub_formatted_string.repeat(repeat_times.into()));
                repeat_times = 0;
            }
            b']' => break,
            _ => formatted_string.push(char.into()),
        }
    }

    formatted_string
}
// @lc code=end
