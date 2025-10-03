/*
 * @lc app=leetcode.cn id=287 lang=rust
 *
 * [287] 寻找重复数
 */

// @lc code=start
// 代码逻辑同 142. 环形链表 II
impl crate::Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let nums: Vec<usize> = nums.into_iter().map(|x| x as usize).collect();
        let mut slow = 0;
        let mut fast = 0;

        loop {
            // 等价于 slow = slow.next 和 fast = fast.next.next
            slow = nums[slow];
            fast = nums[fast];
            fast = nums[fast];

            if slow == fast {
                break;
            }
        }

        let mut head = 0;
        while slow != head {
            slow = nums[slow];
            head = nums[head];
        }
        slow as i32 // 入环口即重复元素
    }
}

// @lc code=end
