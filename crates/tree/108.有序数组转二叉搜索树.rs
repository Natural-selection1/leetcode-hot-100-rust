/*
 * @lc app=leetcode.cn id=108 lang=rust
 *
 * [108] 将有序数组转换为二叉搜索树
 */

use crate::TreeNode;
// @lc code=start

use std::cell::RefCell;
use std::rc::Rc;

impl crate::Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        dfs(&nums)
    }
}

fn dfs(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    let mid_index = nums.len().midpoint(0);

    Some(Rc::new(RefCell::new(TreeNode {
        val: nums[mid_index],
        left: dfs(&nums[..mid_index]),
        right: dfs(&nums[mid_index + 1..]),
    })))
}
// @lc code=end
