/*
 * @lc app=leetcode.cn id=199 lang=rust
 *
 * [199] 二叉树的右视图
 */

// @lc code=start
use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl crate::Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vec_right_view = Vec::with_capacity(100);
        dfs(&root, &mut 0, &mut vec_right_view);
        vec_right_view
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: &mut usize, vec_right_view: &mut Vec<i32>) {
    let Some(node) = node else { return };
    let node = node.borrow();

    if vec_right_view.len() == *depth {
        vec_right_view.push(node.val);
    }
    *depth += 1;

    dfs(&node.right, depth, vec_right_view);
    dfs(&node.left, depth, vec_right_view);

    *depth -= 1;
}
// @lc code=end
