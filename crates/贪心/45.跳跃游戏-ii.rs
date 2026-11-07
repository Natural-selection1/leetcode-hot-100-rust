/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * [45] 跳跃游戏 II
 *
 * https://leetcode.cn/problems/jump-game-ii/description/
 *
 * algorithms
 * Medium (45.62%)
 * Likes:    3087
 * Dislikes: 0
 * Total Accepted:    1.3M
 * Total Submissions: 2.9M
 * Testcase Example:  '[2,3,1,1,4]'
 *
 * 给定一个长度为 n 的 0 索引整数数组 nums。初始位置在下标 0。
 *
 * 每个元素 nums[i] 表示从索引 i 向后跳转的最大长度。换句话说，如果你在索引 i 处，你可以跳转到任意 (i + j) 处：
 *
 *
 * 0 <= j <= nums[i] 且
 * i + j < n
 *
 *
 * 返回到达 n - 1 的最小跳跃次数。测试用例保证可以到达 n - 1。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: nums = [2,3,1,1,4]
 * 输出: 2
 * 解释: 跳到最后一个位置的最小跳跃数是 2。
 * 从下标为 0 跳到下标为 1 的位置，跳 1 步，然后跳 3 步到达数组的最后一个位置。
 *
 *
 * 示例 2:
 *
 *
 * 输入: nums = [2,3,0,1,4]
 * 输出: 2
 *
 *
 *
 *
 * 提示:
 *
 *
 * 1 <= nums.length <= 10^4
 * 0 <= nums[i] <= 1000
 * 题目保证可以到达 n - 1
 *
 *
 */

// @lc code=start
impl crate::Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let nums: Vec<usize> = nums.into_iter().map(|x| x as usize).collect();
        let mut times = 0;
        let mut next_round_furest = 0;
        let mut current_round_furest = 0;

        for i in 0..nums.len() -1 {
            next_round_furest = next_round_furest.max(i + nums[i]);

            if i == current_round_furest {
                current_round_furest = next_round_furest;
                times += 1;
            }
        }

        times
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

pub fn jump(nums: Vec<i32>) -> i32 {
    let nums: Vec<usize> = nums.into_iter().map(|x| x as usize).collect();
    let mut answer = 0;

    nums.iter().enumerate().take(nums.len() - 1).fold(
        (0, 0),
        |(current_round_reachable_index, next_round_reachable_index),
         (current_index, &jump_distance)| {
            let next_round_reachable_index =
                next_round_reachable_index.max(current_index + jump_distance);

            match current_index != current_round_reachable_index {
                true => (current_round_reachable_index, next_round_reachable_index),
                false => {
                    answer += 1;
                    (next_round_reachable_index, next_round_reachable_index)
                }
            }
        },
    );

    answer
}

*/
