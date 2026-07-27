/*
 * @lc app=leetcode.cn id=236 lang=rust
 *
 * [236] 二叉树的最近公共祖先
 */

// @lc code=start
use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
use std::unreachable;

impl crate::Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        node_1: Option<Rc<RefCell<TreeNode>>>,
        node_2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        dfs(root, node_1.unwrap(), node_2.unwrap())
    }
}

fn dfs(
    root: Option<Rc<RefCell<TreeNode>>>,
    node_1: Rc<RefCell<TreeNode>>,
    node_2: Rc<RefCell<TreeNode>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let root = root?;

    let root_left = root.borrow_mut().left.take();
    let root_right = root.borrow_mut().right.take();

    let left = dfs(root_left, node_1.clone(), node_2.clone());
    let right = dfs(root_right, node_1.clone(), node_2.clone());

    match (left, right, (root == node_1 || root == node_2)) {
        (None, None, false) => None,
        (None, None, true) => Some(root),
        (Some(_), Some(_), false) => Some(root),
        (Some(_), None, true) | (None, Some(_), true) => Some(root),
        (node @ Some(_), None, false) | (None, node @ Some(_), false) => node,
        (Some(_), Some(_), true) => unreachable!(),
    }
}
// @lc code=end
