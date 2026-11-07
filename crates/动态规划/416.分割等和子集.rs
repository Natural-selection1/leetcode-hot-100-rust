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
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 != 0 { return false }
        if nums.len() == 1 { return false }

        let half_sum = sum / 2;
        let mut nums: Vec<usize> = nums
            .into_iter()
            .filter(|num| *num <= half_sum)
            .map(|num| num as usize)
            .collect();
        nums.sort();

        // dp数组含义:
        // 对于数值 j, 任取 nums[..=i] 中的数字是否能够凑出, 布尔值为 dp[i][j]

        // 递推公式:
        // dp[i][j] =
        //       dp[i - 1][j] // 不用当前数
        //    || dp[i][j - i] // 使用当前数

        // 初始化:
        //   0 1 2 3 4 ... n
        // 2 t
        // 3 t
        // 5 t

        let length = (half_sum + 1) as usize;

        let mut dp = vec![false; length];
        dp[0] = true;

        for num in nums {
            for j in (num..length).rev() {
                dp[j] = dp[j] || dp[j - num];
            }
        }

        // fprint(&dp);
        dp[length - 1]
    }
}

use std::fmt::Debug;
fn fprint<T: Debug>(dp: &Vec<Vec<T>>) {
    for row in dp {
        println!("{:?}", row);
    }

    println!();
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
