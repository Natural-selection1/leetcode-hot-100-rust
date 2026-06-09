/*
 * @lc app=leetcode.cn id=283 lang=rust
 *
 * [283] 移动零
 */

// @lc code=start
impl crate::Solution {
    pub fn move_zeroes(nums: &mut [i32]) {
        let mut next_not_zero_num_index = 0;

        for index in 0..nums.len() {
            if nums[index] != 0 {
                if index != next_not_zero_num_index {
                    nums.swap(index, next_not_zero_num_index);
                }
                next_not_zero_num_index += 1;
            }
        }
    }
}
// @lc code=end

#[test]
fn acm_mode_main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let nums = lines.next().unwrap().unwrap();
    let mut nums: Vec<i32> = nums
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    crate::Solution::move_zeroes(&mut nums);
    println!("{:?}", nums)
}
