/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 *
 * https://leetcode.cn/problems/climbing-stairs/description/
 *
 * algorithms
 * Easy (55.63%)
 * Likes:    4114
 * Dislikes: 0
 * Total Accepted:    2.3M
 * Total Submissions: 4.1M
 * Testcase Example:  '2'
 *
 * 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
 *
 * 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 2
 * 输出：2
 * 解释：有两种方法可以爬到楼顶。
 * 1. 1 阶 + 1 阶
 * 2. 2 阶
 *
 * 示例 2：
 *
 *
 * 输入：n = 3
 * 输出：3
 * 解释：有三种方法可以爬到楼顶。
 * 1. 1 阶 + 1 阶 + 1 阶
 * 2. 1 阶 + 2 阶
 * 3. 2 阶 + 1 阶
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n <= 45
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn climb_stairs(times: i32) -> i32 {
        let times = times as usize;

        // dp数组含义:
        // 爬到第n阶时, 有 dp[n] 种方式

        // 递推公式:
        // dp[n] = dp[n-1] + dp[n-2]

        // dp数组初始化
        let mut dp = vec![0; (times + 1) as usize];
        dp[0] = 1; // 这里很重要!
        dp[1] = 1;

        for i in 2..(times + 1) {
            dp[i] = dp[i - 1] + dp[i - 2]
        }

        dp[times]
    }
}
// @lc code=end

#[test]
fn main() {
    use std::io::BufRead;
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines();

    let first_line = lines.next().unwrap().unwrap().parse::<i32>().unwrap();

    eprintln!("\nanswer = {}\n", Solution::climb_stairs(first_line));
}

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

pub fn climb_stairs(times: i32) -> i32 {
        (1..=times)
            .fold(
                (0, 1),
                // 状态转移:
                // dp[i]       = dp[i-2] + dp[i-1]
                |(n1, n2), _| {
                    // dp[i+1] = dp[i-1] + (dp[i-2] + dp[i-1])
                    (n2, n1 + n2)
                },
            )
            .1
    }
}
*/
