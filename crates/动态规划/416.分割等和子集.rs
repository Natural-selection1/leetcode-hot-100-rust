/*
 * @lc app=leetcode.cn id=416 lang=rust
 *
 * [416] 分割等和子集
 *
 * https://leetcode.cn/problems/partition-equal-subset-sum/description/
 *
 * algorithms
 * Medium (54.05%)
 * Likes:    2595
 * Dislikes: 0
 * Total Accepted:    1.1M
 * Total Submissions: 1.9M
 * Testcase Example:  '[1,5,11,5]'
 *
 * 给你一个 只包含正整数 的 非空 数组 nums 。请你判断是否可以将这个数组分割成两个子集，使得两个子集的元素和相等。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,5,11,5]
 * 输出：true
 * 解释：数组可以分割成 [1, 5, 5] 和 [11] 。
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1,2,3,5]
 * 输出：false
 * 解释：数组不能分割成两个元素和相等的子集。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        todo!()
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

pub fn can_partition(nums: Vec<i32>) -> bool {
        let nums: Vec<usize> = nums.into_iter().map(|num| num as usize).collect();
        let nums_sum: usize = nums.iter().sum();
        let target_sum = nums_sum / 2;

        // 可行性检查
        if !nums_sum.is_multiple_of(2) || nums.iter().max() > Some(&target_sum) {
            return false;
        }

        let mut can_build_sum = vec![false; target_sum + 1];
        can_build_sum[0] = true;

        // 其实就是0-1背包
        for num in nums {
            for sum in (0..=target_sum).rev() {
                if sum >= num {
                    can_build_sum[sum] |= can_build_sum[sum - num];
                }
            }

            if can_build_sum[target_sum] {
                return true;
            }
        }

        can_build_sum[target_sum]
    }
}
*/
