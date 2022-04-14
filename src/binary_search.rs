//! # Binary Search / easy(leet code)
//! [문제 링크](https://leetcode.com/problems/binary-search/)
//!
//! Given an array of integers `nums` which is sorted in ascending order, and an integer `target`, write a function to search `target` in `nums`. If `target` exists, then return its index. Otherwise, return `-1`.
//!
//! You must write an algorithm with `O(log n)` runtime complexity.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [-1,0,3,5,9,12], target = 9
//! Output: 4
//! Explanation: 9 exists in nums and its index is 4
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [-1,0,3,5,9,12], target = 2
//! Output: -1
//! Explanation: 2 does not exist in nums so return -1
//!
//! ```
//!
//! **Constraints:**
//!
//! -   `1 <= nums.length <= 104`
//! -   `-104 < nums[i], target < 104`
//! -   All the integers in `nums` are **unique**.
//! -   `nums` is sorted in ascending order.
#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn case_1() {
		assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
	}
	#[test]
	fn case_2() {
		assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
	}
	#[test]
	fn case_3() {
		assert_eq!(Solution::search(vec![5], 5), 0);
	}
}

pub struct Solution {}

impl Solution {
	pub fn search(nums: Vec<i32>, target: i32) -> i32 {
		let mut left = 0;
		let mut right = (nums.len() - 1) as i32;
		while left <= right {
			let pivot = left + (right - left) / 2;
			let cur_val = nums[pivot as usize];
			if cur_val == target {
				return pivot as i32;
			} else if cur_val < target {
				left = pivot + 1;
			} else {
				right = pivot - 1;
			}
		}
		-1
	}

	pub fn search_best_prac(nums: Vec<i32>, target: i32) -> i32 {
		use std::cmp::Ordering;
		let (mut low, mut high) = (0, nums.len());
		while low < high {
			let mid = low + (high - low) / 2;
			match nums[mid].cmp(&target) {
				Ordering::Equal => return mid as i32,
				Ordering::Less => low = mid + 1,
				Ordering::Greater => high = mid,
			}
		}
		-1
	}
}
