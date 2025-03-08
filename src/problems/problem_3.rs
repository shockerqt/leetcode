use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut cur_len = 0;

        let mut map: HashMap<char, i32> = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            let last_appear = map.get(&c).unwrap_or(&-1);
            cur_len = std::cmp::min(i as i32 - last_appear, cur_len + 1);
            max_len = std::cmp::max(max_len, cur_len);
            map.insert(c, i as i32);
        }

        max_len
    }
}

pub fn example() {
    let test_inputs = ["abcabcbb", "bbbbb", "pwwkew", "abcd", " ", "aab"];
    let test_outputs = [3, 1, 3, 4, 1, 2];

    for (&input, &output) in test_inputs.iter().zip(test_outputs.iter()) {
        let input_string = String::from(input);
        let result = Solution::length_of_longest_substring(input_string);
        assert_eq!(result, output);
    }
}
