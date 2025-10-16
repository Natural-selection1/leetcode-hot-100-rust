/*
 * @lc app=leetcode.cn id=236 lang=rust
 *
 * [236] 二叉树的最近公共祖先
 */

use crate::TreeNode;
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;

impl crate::Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        node_1: Option<Rc<RefCell<TreeNode>>>,
        node_2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root == node_1 || root == node_2 {
            return root;
        }
        let root = root?;

        let left = Self::lowest_common_ancestor(
            root.borrow_mut().left.take(),
            node_1.clone(),
            node_2.clone(),
        );
        let right = Self::lowest_common_ancestor(root.borrow_mut().right.take(), node_1, node_2);

        match (&left, &right) {
            (Some(_), Some(_)) => Some(root), // 左右都找到, 当前节点是最近公共祖先
            // 如果只有单边找到，就返回那一边的返回值
            (None, Some(_)) => right,
            (Some(_), None) => left,
            // 都没有找到，就返回 None
            (None, None) => None,
        }
    }
}

// @lc code=end
