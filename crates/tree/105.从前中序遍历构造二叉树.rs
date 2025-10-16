/*
 * @lc app=leetcode.cn id=105 lang=rust
 *
 * [105] 从前序与中序遍历序列构造二叉树
 */

// @lc code=start
#![allow(non_upper_case_globals)]
#![allow(static_mut_refs)]
use crate::TreeNode;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

static mut preorder: Vec<i32> = vec![];
static mut val_to_in_index_map: BTreeMap<i32, usize> = BTreeMap::new();

impl crate::Solution {
    pub fn build_tree(preorder_: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // let root_val = preorder[0];
        // let root_index_in_pre_order = 0_usize;
        // let root_index_in_in_order = inorder.iter().position(|val| val == &root_val)?;
        // let mut root_node = Some(Rc::new(RefCell::new(TreeNode::new(root_val))));
        // build_node(
        //     &mut root_node,
        //     &preorder,
        //     &inorder,
        //     root_index_in_pre_order,
        //     root_index_in_in_order,
        //     -1,
        //     preorder.len() as i32,
        // );
        // root_node

        let nums_size = preorder_.len();
        unsafe { preorder = preorder_ };

        for (node_in_index, &node_val) in inorder.iter().enumerate() {
            unsafe { val_to_in_index_map.insert(node_val, node_in_index) };
        }

        dfs(0, nums_size, 0)
    }
}

fn dfs(
    node_pre_index: usize,
    pre_right_boundary: usize,
    in_left_boundary: usize, // * 用来算左子树长度的
) -> Option<Rc<RefCell<TreeNode>>> {
    if node_pre_index == pre_right_boundary {
        return None;
    }
    let node_val = unsafe { preorder[node_pre_index] };
    let node_in_index = unsafe { val_to_in_index_map[&node_val] };

    let l_tree_size = node_in_index - in_left_boundary;
    let l_node_pre_index = node_pre_index + 1;
    let r_node_pre_index = l_node_pre_index + l_tree_size;

    let l_node = dfs(l_node_pre_index, r_node_pre_index, in_left_boundary);

    let r_tree_in_left_boundary = node_in_index + 1;
    let r_node = dfs(
        r_node_pre_index,
        pre_right_boundary,
        r_tree_in_left_boundary,
    );

    Some(Rc::new(RefCell::new(TreeNode {
        val: unsafe { preorder[node_pre_index] },
        left: l_node,
        right: r_node,
    })))
}

// fn build_node(
//     node: &mut Option<Rc<RefCell<TreeNode>>>,
//     preorder: &Vec<i32>,
//     inorder: &Vec<i32>,
//     node_pre_index: usize,
//     node_mid_index: usize,
//     left_boundary: i32,
//     right_boundary: i32,
// ) {
//     let Some(node) = node else {
//         return;
//     };
//     let mut node = node.borrow_mut();
//     let next_node_pre_index = node_pre_index + 1;
//     // * 当前节点先序的右边遇到边界
//     if next_node_pre_index >= right_boundary as usize {
//         return;
//     }
//     let Some(next_node_val) = preorder.get(next_node_pre_index) else {
//         return;
//     };
//     let mut next_node = Some(Rc::new(RefCell::new(TreeNode::new(*next_node_val))));
//     // * 判别这是左节点还是右节点
//     let next_node_mid_index = inorder.iter().position(|val| val == next_node_val).unwrap();
//     // * 若在根的右侧, 说明左侧是空, 优先处理一下
//     match next_node_mid_index > node_mid_index {
//         true => {
//             node.left = None;
//             build_node(
//                 &mut next_node,
//                 preorder,
//                 inorder,
//                 next_node_pre_index,
//                 next_node_mid_index,
//                 node_mid_index as i32,
//                 right_boundary,
//             );
//             node.right = next_node;
//         }
//         false => {
//             let 向右跨度 = node_mid_index - left_boundary as usize;
//             let right_node_pre_index = node_pre_index + 向右跨度;
//             if right_node_pre_index >= right_boundary as usize {
//                 node.right = None;
//                 build_node(
//                     &mut next_node,
//                     preorder,
//                     inorder,
//                     next_node_pre_index,
//                     next_node_mid_index,
//                     left_boundary,
//                     node_mid_index as i32,
//                 );
//                 node.left = next_node;
//             }
//             todo!()
//         }
//     }
// }

// @lc code=end
