/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 *
 * https://leetcode.cn/problems/3sum/description/
 *
 * algorithms
 * Medium (40.28%)
 * Likes:    8161
 * Dislikes: 0
 * Total Accepted:    3.1M
 * Total Submissions: 7.6M
 * Testcase Example:  '[-1,0,1,2,-1,-4]'
 *
 * 给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j !=
 * k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请你返回所有和为 0 且不重复的三元组。
 *
 * 注意：答案中不可以包含重复的三元组。
 *
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [-1,0,1,2,-1,-4]
 * 输出：[[-1,-1,2],[-1,0,1]]
 * 解释：
 * nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
 * nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
 * nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
 * 不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
 * 注意，输出的顺序和三元组的顺序并不重要。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [0,1,1]
 * 输出：[]
 * 解释：唯一可能的三元组和不为 0 。
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [0,0,0]
 * 输出：[[0,0,0]]
 * 解释：唯一可能的三元组和为 0 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 3 <= nums.length <= 3000
 * -10^5 <= nums[i] <= 10^5
 *
 *
 */

// @lc code=start


impl crate::Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 固定一个数, 做剩下的两数之和
        nums.sort();
        let mut result = vec![];

        for l_idx in 0..nums.len() - 2 {
            let l_val = nums[l_idx];

            if l_val > 0 { break; }
            if l_idx >= 1 && l_val == nums[l_idx - 1] { continue; }

            let mut m_idx = l_idx + 1;
            let mut r_idx = nums.len() - 1;

            while m_idx < r_idx {
                let m_val = nums[m_idx];
                let r_val = nums[r_idx];

                // mid 也要去重 (不过可以与 l 相同)
                if m_idx > l_idx + 1 && m_val == nums[m_idx - 1] {
                    m_idx += 1;
                    continue;
                }

                let sum = l_val + m_val + r_val;

                match sum.cmp(&0) {
                    std::cmp::Ordering::Equal => {
                        result.push(vec![l_val as i32, m_val as i32, r_val as i32]);
                        m_idx += 1;
                        r_idx -= 1;
                    }
                    std::cmp::Ordering::Less => { m_idx += 1 }
                    std::cmp::Ordering::Greater => { r_idx -= 1 }
                }
            }
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

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let length = nums.len();
    let mut result = vec![];
    let max_two_sum = nums[length - 2] + nums[length - 1];

    for (l_index, &l_val) in nums.iter().enumerate().take(length - 2) {
        // 因为从左向右以最小值为基准遍历
        // 所以当最小的都大于0时, 则三个数都必然大于0了
        if l_val > 0 {
            break;
        }
        // 加上最大的两个数都不超过0, 或者这个数字已经找过一次了
        if max_two_sum + l_val < 0 || nums.get(l_index - 1) == Some(&l_val) {
            continue;
        }

        // 把 `三数之和` 简化为 `两数之和`
        let mut m_index = l_index + 1;
        let mut r_index = length - 1;

        while m_index < r_index {
            let m_val = nums[m_index];
            let r_val = nums[r_index];
            let sum = l_val + m_val + r_val;

            match sum.cmp(&0) {
                Ordering::Less => m_index += 1,
                Ordering::Greater => r_index -= 1,
                Ordering::Equal => {
                    // 只有中值与上一个不同时(那么另一个数也肯定不同)
                    // 或者这是 -1, -1, 2 这种情况才记录
                    if m_val != nums[m_index - 1] || m_index == l_index + 1 {
                        result.push(vec![l_val, m_val, r_val]);
                    }
                    m_index += 1;
                    r_index -= 1;
                }
            }
        }
    }

    result
}

*/
