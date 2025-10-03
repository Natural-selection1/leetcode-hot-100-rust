/*
 * @lc app=leetcode.cn id=347 lang=rust
 *
 * [347] 前 K 个高频元素
 */

// @lc code=start
use std::collections::HashMap;
impl crate::Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let count_map = nums.into_iter().fold(HashMap::new(), |mut count_map, num| {
            *count_map.entry(num).or_insert(0) += 1;
            count_map
        });
        #[allow(clippy::unwrap_used, reason = "count_map不为空, 至少是0")]
        let max_count = *count_map.values().max().unwrap();

        // 第二步：把出现次数相同的元素，放到同一个桶中
        let buckets =
            count_map
                .into_iter()
                .fold(vec![vec![]; max_count + 1], |mut buckets, (num, count)| {
                    buckets[count].push(num);
                    buckets
                });

        // 第三步：倒序遍历 buckets，把出现次数前 k 大的元素加入答案
        let k = k as usize;
        let mut answer = Vec::with_capacity(k);
        for bucket in buckets.into_iter().rev() {
            answer.extend(bucket);
            // 题目保证答案唯一，一定会出现恰好等于 k 的情况
            if answer.len() == k {
                return answer;
            }
        }
        unreachable!()
    }
}
// @lc code=end
