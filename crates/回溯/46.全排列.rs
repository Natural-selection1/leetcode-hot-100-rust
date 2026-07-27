/*
 * @lc app=leetcode.cn id=46 lang=rust
 *
 * [46] 全排列
 */

// @lc code=start

impl crate::Solution {
    // used 数组
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        let mut used = vec![false; nums.len()];
        backtracking(&mut result, &mut path, &nums, &mut used);
        result
    }

    // 原地修改
    pub fn permute_(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut full_permutation: Vec<Vec<i32>> = Vec::new();
        dfs(0, &mut nums, &mut full_permutation);
        full_permutation
    }
}

fn backtracking(
    result: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
    nums: &Vec<i32>,
    used: &mut Vec<bool>,
) {
    let len = nums.len();
    if path.len() == len {
        return result.push(path.clone());
    }

    for i in 0..len {
        if used[i] {
            continue;
        }
        used[i] = true;
        path.push(nums[i]);
        backtracking(result, path, nums, used);
        path.pop();
        used[i] = false;
    }
}

fn dfs(first: usize, nums: &mut Vec<i32>, full_permutation: &mut Vec<Vec<i32>>) {
    if first == nums.len() {
        return full_permutation.push(nums.to_vec());
    }

    for i in first..nums.len() {
        nums.swap(first, i);
        dfs(first + 1, nums, full_permutation);
        nums.swap(first, i);
    }
}
// @lc code=end
