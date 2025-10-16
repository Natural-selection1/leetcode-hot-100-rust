/*
 * @lc app=leetcode.cn id=236 lang=rust
 *
 * [236] 二叉树的最近公共祖先
 */

// @lc code=start
use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
type Node = Rc<RefCell<TreeNode>>;

impl crate::Solution {
    pub fn lowest_common_ancestor(
        root: Option<Node>,
        node_1: Option<Node>,
        node_2: Option<Node>,
    ) -> Option<Node> {
        if root == node_1 || root == node_2 {
            return root;
        }
        let root = root?;
        let root_left = root.borrow_mut().left.take();
        let root_right = root.borrow_mut().right.take();

        let left = Self::lowest_common_ancestor(root_left, node_1.clone(), node_2.clone());
        let right = Self::lowest_common_ancestor(root_right, node_1, node_2);

        match (&left, &right) {
            // 都没有找到，就返回 None
            (None, None) => None,
            // 左右都找到, 当前节点是最近公共祖先
            (Some(_), Some(_)) => Some(root),
            // 如果只有单边找到，就返回那一边的返回值
            (None, Some(_)) => right,
            (Some(_), None) => left,
        }
    }
}
// @lc code=end
