/*
 * @lc app=leetcode.cn id=102 lang=rust
 *
 * [102] 二叉树的层序遍历
 */

// @lc code=start
use crate::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
type Node = Rc<RefCell<TreeNode>>;

impl crate::Solution {
    pub fn level_order(root: Option<Node>) -> Vec<Vec<i32>> {
        let Some(root) = root else {
            return vec![];
        };

        let mut answer = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root);

        loop {
            let current_queue_len = queue.len();
            if current_queue_len == 0 {
                break;
            }
            let mut node_in_this_layer = vec![];

            for _ in 0..current_queue_len {
                let node = queue.pop_front().unwrap();
                let mut node = node.borrow_mut();

                if let Some(left_node) = node.left.take() {
                    queue.push_back(left_node);
                }
                if let Some(right_node) = node.right.take() {
                    queue.push_back(right_node);
                }
                node_in_this_layer.push(node.val);
            }
            answer.push(node_in_this_layer);
        }

        answer

        // let mut answer = vec![];
        // let mut node_queue = VecDeque::new();

        // if let Some(root) = root {
        //     node_queue.push_back(root);
        // }

        // while !node_queue.is_empty() {
        //     let layer_len = node_queue.len();
        //     let mut vals = Vec::with_capacity(layer_len); // 预分配空间

        //     for _ in 0..layer_len {
        //         if let Some(node) = node_queue.pop_front() {
        //             let mut x = node.borrow_mut();
        //             vals.push(x.val);
        //             if let Some(left) = x.left.take() {
        //                 node_queue.push_back(left);
        //             }
        //             if let Some(right) = x.right.take() {
        //                 node_queue.push_back(right);
        //             }
        //         }
        //     }
        //     answer.push(vals);
        // }
        // answer
    }
}

// @lc code=end
