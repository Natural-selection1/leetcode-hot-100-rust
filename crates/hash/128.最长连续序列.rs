/*
 * @lc app=leetcode.cn id=128 lang=rust
 *
 * [128] 最长连续序列
 */
use crate::Solution;
// @lc code=start
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let set: HashSet<&i32> = HashSet::from_iter(&nums);
        let half_of_set_lenth = (set.len() / 2 + 1) as i32;
        let mut longest = 0;

        for &num in &set {
            // * 我们需要找到某一个序列的起点, 而不是整个集合的起点
            if set.contains(&(num - 1)) {
                continue;
            }
            // * 现在, 我们位于某一序列的起点, 检查、直到元素不存在
            let mut num_now = num + 1;
            while set.contains(&num_now) {
                num_now += 1;
            }
            // * 巡逻完了某一序列, 现在更新最长数据
            longest = longest.max(num_now - num);

            if longest >= half_of_set_lenth {
                break;
            }
        }

        longest
    }
}
// @lc code=end
