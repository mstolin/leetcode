#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    // This solution requires a time time complexity of O(n^2) and
    // has a space complexity of O(1), because space doesn't depend on input
    pub fn two_sum_x(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (index_a, num_a) in nums.iter().enumerate() {
            for (index_b, num_b) in nums[index_a + 1..].iter().enumerate() {
                if *num_a + *num_b == target {
                    let next_index = (index_a + index_b) + 1;
                    return vec![index_a as i32, next_index as i32];
                }
            }
        }
        vec![]
    }

    // This solution requires a time complexity of O(n)
    // but has a space complexity of O(n), it stores at most n items
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache: HashMap<i32, usize> = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            let diff = target - *num;
            if let Some(prev_index) = cache.get(&diff) {
                return vec![*prev_index as i32, index as i32];
            }
            cache.insert(*num, index);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![0, 4, 3, 0], 0), vec![0, 3]);
        assert_eq!(Solution::two_sum(vec![-3, 4, 3, 90], 0), vec![0, 2]);
    }
}
