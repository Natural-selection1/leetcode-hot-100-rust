use crate::Solution;

impl Solution {
    fn selection_sort(nums: &mut Vec<i32>) {
        for i in 0..nums.len() {
            let mut index_of_min_num = i;

            for j in (i + 1)..nums.len() {
                if nums[j] < nums[index_of_min_num] {
                    index_of_min_num = j;
                }
            }

            nums.swap(i, index_of_min_num);
        }
    }
}
