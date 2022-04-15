//! # Search insert position / Easy(leet code)
//! [문제 링크](https://leetcode.com/problems/search-insert-position/)
//!
//! Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
//!
//! You must write an algorithm with `O(log n)` runtime complexity.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [1,3,5,6], target = 5
//! Output: 2
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [1,3,5,6], target = 2
//! Output: 1
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums = [1,3,5,6], target = 7
//! Output: 4
//!
//! ```
//!
//! **Constraints:**
//!
//! -   `1 <= nums.length <= 104`
//! -   `-104 <= nums[i] <= 104`
//! -   `nums` contains **distinct** values sorted in **ascending** order.
//! -   `-104 <= target <= 104`

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn case_1() {
		assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
	}
	#[test]
	fn case_2() {
		assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
	}

	#[test]
	fn case_3() {
		assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
	}
}

use std::cmp::Ordering;
pub struct Solution {}

impl Solution {
	pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
		let mut left: i32 = 0;
		let mut right: i32 = (nums.len() - 1) as i32;
		while left <= right {
			let pivot = left + (right - left) / 2;
			match nums[pivot as usize].cmp(&target) {
				Ordering::Equal => return pivot as i32,
				Ordering::Less => left = pivot + 1,
				Ordering::Greater => right = pivot - 1,
			}
		}
		(right + 1) as i32
	}

	/// # 심플 솔루션
	/// 러스트에서 제공하는 Binary Search에서 찾지 못하면, 삽입 가능한 index 를 리턴하게 설계되어있다.
	/// > If the value is not found then [Result::Err] is returned, containing the index where a matching element could be inserted while maintaining sorted order.
	pub fn search_insert_simple(nums: Vec<i32>, target: i32) -> i32 {
		nums.binary_search(&target).unwrap_or_else(|x| x) as i32
	}
}
