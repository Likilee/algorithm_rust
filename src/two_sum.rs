//! # Two Sum / Easy(leet code)
//! [문제 링크](https://leetcode.com/problems/two-sum/)
//!
//! Given an array of integers `nums` and an integer `target`, return _indices of the two numbers such that they add up to `target`_.
//!
//! You may assume that each input would have **_exactly_ one solution**, and you may not use the _same_ element twice.
//!
//! You can return the answer in any order.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [2,7,11,15], target = 9
//! Output: [0,1]
//! Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [3,2,4], target = 6
//! Output: [1,2]
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums = [3,3], target = 6
//! Output: [0,1]
//!
//! ```
//!
//! **Constraints:**
//!
//! -   `2 <= nums.length <= 104`
//! -   `-109 <= nums[i] <= 109`
//! -   `-109 <= target <= 109`
//! -   **Only one valid answer exists.**
//!
//! **Follow-up:** Can you come up with an algorithm that is less than `O(n2)` time complexity?

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn case_1() {
		assert_eq!(two_sum(vec![2, 7, 11, 15], 9), [0, 1])
	}
	#[test]
	fn case_2() {
		assert_eq!(two_sum(vec![3, 2, 4], 6), [1, 2])
	}
	#[test]
	fn case_3() {
		assert_eq!(two_sum(vec![3, 3], 6), [0, 1])
	}

	#[test]
	fn best_prac_case_1() {
		assert_eq!(two_sum_best_prac(vec![2, 7, 11, 15], 9), [1, 0])
	}

	#[test]
	fn best_prac_case_2() {
		assert_eq!(two_sum_best_prac(vec![3, 2, 4], 6), [2, 1])
	}
	#[test]
	fn best_prac_case_3() {
		assert_eq!(two_sum_best_prac(vec![3, 3], 6), [1, 0])
	}
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
	//합이 target 과 일치하는 조합 찾기.
	let mut i = 0;
	let mut j = 1;
	loop {
		if j == nums.len() {
			i += 1;
			j = i + 1;
		}
		if nums[i] + nums[j] == target {
			return vec![i as i32, j as i32];
		} else {
			j += 1;
		}
	}
}

pub fn two_sum_best_prac(nums: Vec<i32>, target: i32) -> Vec<i32> {
	use std::collections::HashMap;

	let mut m: HashMap<i32, i32> = HashMap::new();
	for (i, v) in nums.iter().enumerate() {
		match m.get(&(target - *v)) {
			Some(&i2) => return vec![i as i32, i2],
			None => m.insert(*v, i as i32),
		};
		println!("{:?}", m);
	}
	vec![]
}
