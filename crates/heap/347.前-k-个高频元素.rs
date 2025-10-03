/*
 * @lc app=leetcode.cn id=347 lang=rust
 *
 * [347] 前 K 个高频元素
 */

// @lc code=start
use std::collections::HashMap;
impl crate::Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count_map = HashMap::new();
        nums.into_iter()
            .for_each(|x| *count_map.entry(x).or_insert(0) += 1);
        let max_count = *count_map.values().max().unwrap();

        // 第二步：把出现次数相同的元素，放到同一个桶中
        let mut buckets = vec![vec![]; max_count + 1];
        for (num, count) in count_map {
            buckets[count].push(num);
        }

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
