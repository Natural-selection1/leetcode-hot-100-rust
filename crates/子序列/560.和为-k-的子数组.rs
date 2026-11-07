/*
 * @lc app=leetcode.cn id=560 lang=rust
 *
 * [560] 和为 K 的子数组
 *
 * https://leetcode.cn/problems/subarray-sum-equals-k/description/
 *
 * algorithms
 * Medium (46.26%)
 * Likes:    3223
 * Dislikes: 0
 * Total Accepted:    1.2M
 * Total Submissions: 2.5M
 * Testcase Example:  '[1,1,1]\n2'
 *
 * 给你一个整数数组 nums 和一个整数 k ，请你统计并返回 该数组中和为 k 的子数组的个数 。
 *
 * 子数组是数组中元素的连续非空序列。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,1,1], k = 2
 * 输出：2
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1,2,3], k = 3
 * 输出：2
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 2 * 10^4
 * -1000 <= nums[i] <= 1000
 * -10^7 <= k <= 10^7
 *
 *
 */

// @lc code=start

impl crate::Solution {
    pub fn subarray_sum(nums: Vec<i32>, target: i32) -> i32 {
        use std::collections::HashMap;

        let mut prefix_sum = 0;
        let mut result = 0;
        let mut prefix_sum_count_map: HashMap<i32, i32> = HashMap::new();
        prefix_sum_count_map.insert(0, 1);

        for index in 0..nums.len() {
            let num = nums[index];
            prefix_sum += num;
            let diff = prefix_sum - target;

            if let Some(count) = prefix_sum_count_map.get(&diff) {
                result += count;
            }

            prefix_sum_count_map.entry(prefix_sum).and_modify(|count|* count += 1).or_insert(1);
        }

        result
    }
}
// @lc code=end

/*
1
2
3
4
5
6
7
8
9
0
1
2
3
4
5
6
7
8
9
0

    pub fn subarray_sum(nums: Vec<i32>, target_sum: i32) -> i32 {
        let mut result = 0;
        let mut prefix_sum = 0;
        let mut sum_count_map = HashMap::new();
        // * 更新当前迭代值后, 先向前搜索, 再插入当前
        // * 为了保证第一个值有值可搜, 必须先插入一个 `0`
        sum_count_map.insert(0, 1);
        for num in nums {
            prefix_sum += num;

            let difference = prefix_sum - target_sum;
            if let Some(count) = sum_count_map.get(&difference) {
                result += count
            }
            *sum_count_map.entry(prefix_sum).or_insert(0) += 1;
        }

        result
    }

*/
