/*
 * @lc app=leetcode.cn id=98 lang=rust
 *
 * [98] 验证二叉搜索树
 */

// @lc code=start
use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

// 两种递归写法
impl crate::Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut is_valid = true;
        dfs(&root, i64::MIN, i64::MAX, &mut is_valid);
        is_valid
    }

    pub fn is_valid_bst_(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        dfs_(&root, i64::MIN, i64::MAX)
    }
}

fn dfs(
    node: &Option<Rc<RefCell<TreeNode>>>,
    left_boundary: i64,
    right_boundary: i64,
    is_valid: &mut bool,
) {
    if !*is_valid {
        return;
    }
    let Some(node) = node else { return };
    let node = node.borrow();
    let node_val = node.val as i64;

    if !(left_boundary + 1..right_boundary).contains(&node_val) {
        return *is_valid = false;
    }
    dfs(&node.left, left_boundary, node_val, is_valid);
    dfs(&node.right, node_val, right_boundary, is_valid);
}

fn dfs_(node: &Option<Rc<RefCell<TreeNode>>>, left_boundary: i64, right_boundary: i64) -> bool {
    let Some(node) = node else { return true };
    let node = node.borrow();
    let node_val = node.val as i64;

    (left_boundary + 1..right_boundary).contains(&node_val)
        && dfs_(&node.left, left_boundary, node_val)
        && dfs_(&node.right, node_val, right_boundary)
}

// @lc code=end
