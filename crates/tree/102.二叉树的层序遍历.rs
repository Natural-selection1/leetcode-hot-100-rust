/*
 * @lc app=leetcode.cn id=102 lang=rust
 *
 * [102] 二叉树的层序遍历
 */

// @lc code=start
#![allow(clippy::option_map_unit_fn)]
use crate::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl crate::Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let root = match root {
            None => return vec![],
            Some(root) => root,
        };
        let mut level_order = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let mut node_in_this_layer = vec![];

            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let mut node = node.borrow_mut();

                node.left.take().map(|node| queue.push_back(node));
                node.right.take().map(|node| queue.push_back(node));
                node_in_this_layer.push(node.val);
            }
            level_order.push(node_in_this_layer);
        }

        level_order
    }
}

// @lc code=end
