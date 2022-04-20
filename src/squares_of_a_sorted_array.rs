//! # 977. Squares of a Sorted Array / Easy(leet code)
//! [문제링크](https://leetcode.com/problems/squares-of-a-sorted-array/)
//!
//! Given an integer array `nums` sorted in **non-decreasing** order, return _an array of **the squares of each number** sorted in non-decreasing order_.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [-4,-1,0,3,10]
//! Output: [0,1,9,16,100]
//! Explanation: After squaring, the array becomes [16,1,0,9,100].
//! After sorting, it becomes [0,1,9,16,100].
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [-7,-3,2,3,11]
//! Output: [4,9,9,49,121]
//!
//! ```
//!
//! **Constraints:**
//!
//! -   `1 <= nums.length <= 104`
//! -   `-104 <= nums[i] <= 104`
//! -   `nums` is sorted in **non-decreasing** order.
//!
//! **Follow up:** Squaring each element and sorting the new array is very trivial, could you find an `O(n)` solution using a different approach?
#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn case_1() {
		let nums = vec![-4, -1, 0, 3, 10];
		assert_eq!(Solution::sorted_squares(nums), vec![0, 1, 9, 16, 100])
	}

	#[test]
	fn case_2() {
		let nums = vec![-7, -3, 2, 3, 11];
		assert_eq!(Solution::sorted_squares(nums), vec![4, 9, 9, 49, 121])
	}
}

pub struct Solution {}

impl Solution {
	pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
		let mut squared: Vec<i32> = nums.iter().map(|number| number * number).collect();
		squared.sort();
		squared
	}
}
