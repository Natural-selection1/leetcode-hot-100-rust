/*
 * @lc app=leetcode.cn id=300 lang=rust
 *
 * [300] 最长递增子序列
 */

// @lc code=start
impl crate::Solution {
    // 动态规划
    pub fn length_of_lis_(nums: Vec<i32>) -> i32 {
        let mut length_of_lis = 1;
        let mut len_when_arrive = vec![1; nums.len()];

        for (r_index, r_num) in nums.iter().enumerate().skip(1) {
            for (l_index, l_num) in nums.iter().enumerate().take(r_index) {
                if l_num < r_num {
                    len_when_arrive[r_index] =
                        len_when_arrive[r_index].max(len_when_arrive[l_index] + 1);
                }
            }
            length_of_lis = length_of_lis.max(len_when_arrive[r_index]);
        }

        length_of_lis
    }

    // 贪心
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut min_tail_num_of_len = vec![i32::MIN, nums[0]];

        for num in nums.into_iter().skip(1) {
            if Some(&num) > min_tail_num_of_len.last() {
                min_tail_num_of_len.push(num);
                continue;
            }
            if let Err(index) = min_tail_num_of_len.binary_search(&num) {
                min_tail_num_of_len[index] = num;
            }
        }

        min_tail_num_of_len.len() as i32 - 1
    }
}
// @lc code=end
