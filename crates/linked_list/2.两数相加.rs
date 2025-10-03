/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 */

// @lc code=start
use crate::ListNode;

impl crate::Solution {
    pub fn add_two_numbers(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = false;
        let mut dummy = ListNode::new(i32::MIN);
        let mut cursor = &mut dummy;

        while let Some(node1) = list1.as_mut()
            && let Some(node2) = list2.as_mut()
        {
            #[rustfmt::skip]
            let num = match (carry, (node1.val + node2.val) as u32) {
                (false, num @ 0..=9) => { carry = false; num      }
                (false, num @ 10.. ) => { carry = true;  num - 10 }
                (true,  num @ 0..=8) => { carry = false; num + 1  }
                (true,  num @ 9..  ) => { carry = true;  num - 9  }
            };
            cursor.next = Some(Box::new(ListNode::new(num as i32)));
            cursor = cursor.next.as_mut().unwrap();
            list1 = node1.next.take();
            list2 = node2.next.take();
        }

        #[rustfmt::skip]
        let mut exist_list = match (list1, list2) {
            (list @ Some(_), None) |
            (None, list @ Some(_)) => list,
            (None, None) => None,
            (Some(_), Some(_)) => unreachable!(),
        };

        while let Some(node) = exist_list.as_mut() {
            #[rustfmt::skip]
            let num = match (carry, node.val as u32) {
                (false, num @ 0..=9) => { carry = false; num      }
                (false, num @ 10.. ) => { carry = true;  num - 10 }
                (true,  num @ 0..=8) => { carry = false; num + 1  }
                (true,  num @ 9..  ) => { carry = true;  num - 9  }
            };
            cursor.next = Some(Box::new(ListNode::new(num as i32)));
            cursor = cursor.next.as_mut().unwrap();
            exist_list = node.next.take();
        }
        carry.then(|| cursor.next = Some(Box::new(ListNode::new(1))));

        dummy.next
    }
}
// @lc code=end
