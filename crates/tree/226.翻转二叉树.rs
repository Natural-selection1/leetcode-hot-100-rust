/*
 * @lc app=leetcode.cn id=226 lang=rust
 *
 * [226] 翻转二叉树
 */

use crate::TreeNode;
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;

impl crate::Solution {
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        dfs_invert_tree(&mut root);
        root
    }
}

fn dfs_invert_tree(node: &mut Option<Rc<RefCell<TreeNode>>>) {
    let Some(node) = node else { return };
    let mut node_borrow = node.borrow_mut();
    dfs_invert_tree(&mut node_borrow.left);
    dfs_invert_tree(&mut node_borrow.right);

    unsafe { std::ptr::swap(&mut node_borrow.left, &mut node_borrow.right) };
}
// @lc code=end
