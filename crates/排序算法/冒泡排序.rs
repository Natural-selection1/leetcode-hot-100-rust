use crate::Solution;

impl Solution {
    fn bubble_sort(nums: &mut Vec<i32>) {
        for i in 0..nums.len() {
            let mut has_swaped = false;
            for j in 0..(nums.len() - i - 1) {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                    has_swaped = true;
                }
            }

            if !has_swaped {
                break;
            }
        }
    }
}
