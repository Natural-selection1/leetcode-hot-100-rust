/*
 * @lc app=leetcode.cn id=98 lang=rust
 *
 * [98] 验证二叉搜索树
 */

// @lc code=start
use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl crate::Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut is_valid = true;
        dfs(&root, i64::MIN, i64::MAX, &mut is_valid);
        is_valid
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

    if !(node_val > left_boundary && node_val < right_boundary) {
        *is_valid = false;
        return;
    }
    dfs(&node.left, left_boundary, node_val, is_valid);
    dfs(&node.right, node_val, right_boundary, is_valid);
}

// * 优雅递归写法
// impl crate::Solution {
//     pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
//         dfs(&root, i64::MIN, i64::MAX)
//     }
// }
// fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, left_boundary: i64, right_boundary: i64) -> bool {
//     let Some(node) = node else {
//         return true;
//     };
//     let node = node.borrow();
//     let current_node_val = node.val as i64;

//     left_boundary < current_node_val
//         && current_node_val < right_boundary
//         && dfs(&node.left, left_boundary, current_node_val)
//         && dfs(&node.right, current_node_val, right_boundary)
// }

// @lc code=end
