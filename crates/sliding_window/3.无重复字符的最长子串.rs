/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
impl crate::Solution {
    pub fn length_of_longest_substring(string: String) -> i32 {
        // use std::collections::HashMap;
        // let string_iter = string.as_bytes();
        // let iter_len = string_iter.len();
        // let mut result = 0;

        // let mut skip_times = 0_usize;
        // 'base_loop: for (base_index, base_char) in string_iter.iter().enumerate() {
        //     if skip_times > 0 {
        //         skip_times -= 1;
        //         continue;
        //     }
        //     // println!("正在以 {base_index}:{base_char} 为基准, 重置local_length为1");
        //     let mut local_length = 1;
        //     let mut set = HashMap::new();
        //     set.insert(base_char, 1);

        //     let mut walk_loop_iter = (1_usize..).zip(string_iter[(base_index + 1)..].iter());
        //     loop {
        //         let Some((current_length, walk_char)) = walk_loop_iter.next() else {
        //             result = result.max(iter_len - base_index);
        //             break;
        //         };
        //         // println!("当前尝试长度为 {current_length}");
        //         match set.insert(walk_char, current_length) {
        //             None => local_length += 1,
        //             Some(exist_index) => {
        //                 // println!("插入 {walk_char} 失败, 额外跳过 {} 次", exist_index - 1);
        //                 skip_times = exist_index - 1;
        //                 result = result.max(local_length);
        //                 // println!("更新最长长度为 {result} ");
        //                 set.clear();
        //                 continue 'base_loop;
        //             }
        //         }
        //     }
        // }

        // result as i32

        // * 滑动窗口写法
        let string_iter = string.as_bytes();
        let mut rusult = 0;

        let mut window = [false; 128];
        let mut l_index = 0;

        for (r_index, &char) in string_iter.iter().enumerate() {
            let char_ascii_num = char as usize;

            // 如果窗口内已经包含 char，那么再加入一个 char 会导致窗口内有重复元素
            // 所以要在加入 char 之前，先移出窗口内的 char
            while window[char_ascii_num] {
                // 窗口内有 char
                window[string_iter[l_index] as usize] = false;
                l_index += 1; // 缩小窗口
            }

            window[char_ascii_num] = true; // 加入 char
            rusult = rusult.max(r_index - l_index + 1); // 更新窗口长度最大值
        }
        rusult as i32
    }
}
// @lc code=end
