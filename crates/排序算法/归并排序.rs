use crate::Solution;

impl Solution {
    fn merge_sort(nums: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

fn merge_two_sorted_vec(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    todo!()
}

/*
1
2
3
4
5
6
7
8
9
0
1
2
3
4
5
6
7
8
9
0

fn merge_sort(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() <= 1 {
        return nums;
    }

    let mid = nums.len() / 2;

    let (left, right) = nums.split_at(mid);
    let sorted_left = Self::merge_sort(left.to_vec());
    let sorted_right = Self::merge_sort(right.to_vec());

    merge_two_sorted_vec(sorted_left, sorted_right)
}

fn merge_two_sorted_vec(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    let mut left_index = 0;
    let mut right_index = 0;
    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            result.push(left[left_index]);
            left_index += 1;
        } else {
            result.push(right[right_index]);
            right_index += 1;
        }
    }

    if left_index == left.len() {
        result.append(&mut right[right_index..].to_vec());
    } else {
        result.append(&mut left[left_index..].to_vec());
    }

    result
}

*/
