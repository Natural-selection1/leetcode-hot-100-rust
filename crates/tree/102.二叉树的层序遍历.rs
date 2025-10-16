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
type Node = Rc<RefCell<TreeNode>>;

impl crate::Solution {
    pub fn level_order(root: Option<Node>) -> Vec<Vec<i32>> {
        let Some(root) = root else { return vec![] };
        let mut level_order = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let mut node_in_this_layer = vec![];

            for _ in 0..queue.len() {
                #[allow(clippy::unwrap_used, reason = "queue 非空")]
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

    pub fn level_order_(root: Option<Node>) -> Vec<Vec<i32>> {
        let mut answer = vec![];
        let mut node_queue = VecDeque::new();

        root.map(|node| node_queue.push_back(node));

        while !node_queue.is_empty() {
            let layer_len = node_queue.len();
            let mut vals = Vec::with_capacity(layer_len); // 预分配空间

            for _ in 0..layer_len {
                if let Some(node) = node_queue.pop_front() {
                    let mut x = node.borrow_mut();
                    vals.push(x.val);
                    if let Some(left) = x.left.take() {
                        node_queue.push_back(left);
                    }
                    if let Some(right) = x.right.take() {
                        node_queue.push_back(right);
                    }
                }
            }
            answer.push(vals);
        }
        answer
    }
}

// @lc code=end
