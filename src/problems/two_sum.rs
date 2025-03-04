use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut complements = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&value) = complements.get(&num) {
                return vec![value as i32, i as i32];
            }
            complements.insert(target - num, i);
        }
        vec![-1, -1]
    }
}

pub fn example() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("Result: {} {}", result[0], result[1]);
}
