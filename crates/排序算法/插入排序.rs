use crate::Solution;

impl Solution {
    fn insertion_sort(nums: &mut [i32]) {
        for i in 1..nums.len() {
            let current = nums[i];
            let mut j = i;
            while j > 0 && nums[j - 1] > current {
                // 大的元素右移一位
                nums[j] = nums[j - 1];
                j -= 1;
            }

            nums[j] = current;
        }
    }
}
