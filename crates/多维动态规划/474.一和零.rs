/*
 * @lc app=leetcode.cn id=474 lang=rust
 *
 * [474] 一和零
 *
 * https://leetcode.cn/problems/ones-and-zeroes/description/
 *
 * algorithms
 * Medium (67.78%)
 * Likes:    1360
 * Dislikes: 0
 * Total Accepted:    316.9K
 * Total Submissions: 467.2K
 * Testcase Example:  '["10","0001","111001","1","0"]\n 5 \n 3 '
 *
 * 给你一个二进制字符串数组 strs 和两个整数 m 和 n 。
 *
 *
 * 请你找出并返回 strs 的最大子集的长度，该子集中 最多 有 m 个 0 和 n 个 1 。
 *
 * 如果 x 的所有元素也是 y 的元素，集合 x 是集合 y 的 子集 。
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：strs = ["10", "0001", "111001", "1", "0"], m = 5, n = 3
 * 输出：4
 * 解释：最多有 5 个 0 和 3 个 1 的最大子集是 {"10","0001","1","0"} ，因此答案是 4 。
 * 其他满足题意但较小的子集包括 {"0001","1"} 和 {"10","1","0"} 。{"111001"} 不满足题意，因为它含 4 个 1
 * ，大于 n 的值 3 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：strs = ["10", "0", "1"], m = 1, n = 1
 * 输出：2
 * 解释：最大的子集是 {"0", "1"} ，所以答案是 2 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= strs.length <= 600
 * 1 <= strs[i].length <= 100
 * strs[i] 仅由 '0' 和 '1' 组成
 * 1 <= m, n <= 100
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let things: Vec<(usize, usize)> = {
            let mut new_strs = vec![];
            for str in strs {
                let mut count_0 = 0;
                let mut count_1 = 0;

                let chars = str.chars();
                for char in chars {
                    if char == '0' {
                        count_0 += 1
                    } else {
                        count_1 += 1
                    }
                }
                if count_0 > m || count_1 > n {
                    continue;
                }

                new_strs.push((count_0 as usize, count_1 as usize))
            }
            new_strs
        };
        let m = m as usize;
        let n = n as usize;

        // 此时 strs 里的都是在数值范围内的值

        /*
        dp数组含义:
        i 个 0 , j 个 1, 最多能装dp[i][j] 个物品

        递推公式:
        dp[i][j] = max(
            dp[i][j], //不选当前物品
            dp[i - strs[k].0][j - strs[k].1] + 1 // 选当前物品
            )

        初始化:
        0下标: dp[0][0] = 0
        非0下标: dp[i][j] = 0
         */

        let mut dp = vec![vec![0; n + 1]; m + 1];

        for thing in &things {
            let count_0 = thing.0;
            let count_1 = thing.1;

            for i in (count_0..m + 1).rev() {
                for j in (count_1..n + 1).rev() {
                    dp[i][j] = dp[i][j].max(dp[i - count_0][j - count_1] + 1)
                }
            }
        }

        dp[m][n]
    }
}
// @lc code=end
