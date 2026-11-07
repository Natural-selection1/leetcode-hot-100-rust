/*
 * @lc app=leetcode.cn id=78 lang=rust
 *
 * [78] 子集
 *
 * https://leetcode.cn/problems/subsets/description/
 *
 * algorithms
 * Medium (82.12%)
 * Likes:    2674
 * Dislikes: 0
 * Total Accepted:    1.3M
 * Total Submissions: 1.6M
 * Testcase Example:  '[1,2,3]'
 *
 * 给你一个整数数组 nums ，数组中的元素 互不相同 。返回该数组所有可能的子集（幂集）。
 *
 * 解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,2,3]
 * 输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [0]
 * 输出：[[],[0]]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 10
 * -10 <= nums[i] <= 10
 * nums 中的所有元素 互不相同
 *
 *
 */

// @lc code=start
impl crate::Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut path: Vec<i32> = vec![];
        let mut result: Vec<Vec<i32>> = vec![];

        backtracing(&nums, 0,&mut path, &mut result);

        result
    }
}

fn backtracing(
    nums: &Vec<i32>,
    start: usize,
    path: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    result.push(path.clone());

    if start > nums.len() {
        return;
    }

    for i in start..nums.len() {
        path.push(nums[i]);
        backtracing(nums, i + 1, path, result);
        path.pop();
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

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut answer = vec![];
    dfs(&nums, 0, &mut vec![false; nums.len()], &mut answer);
    answer
}

fn dfs(nums: &Vec<i32>, index: usize, used: &mut Vec<bool>, answer: &mut Vec<Vec<i32>>) {
    if index == nums.len() {
        return answer.push(
            Iterator::zip(used.iter(), nums)
                .filter_map(|(&is_used, &num)| if is_used { Some(num) } else { None })
                .collect(),
        );
    }

    // 选或不选
    used[index] = true;
    dfs(nums, index + 1, used, answer);
    used[index] = false;

    dfs(nums, index + 1, used, answer);
}

*/
