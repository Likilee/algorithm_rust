//! # First Bad Version / Easy(leet code)
//! [문제 링크](https://leetcode.com/problems/first-bad-version/)
//!
//! You are a product manager and currently leading a team to develop a new product. Unfortunately, the latest version of your product fails the quality check. Since each version is developed based on the previous version, all the versions after a bad version are also bad.
//!
//! Suppose you have n versions [1, 2, ..., n] and you want to find out the first bad one, which causes all the following ones to be bad.
//!
//! You are given an API bool isBadVersion(version) which returns whether version is bad. Implement a function to find the first bad version. You should minimize the number of calls to the API.
//!
//!
//!
//! Example 1:
//!
//! Input: n = 5, bad = 4
//! Output: 4
//! Explanation:
//! call isBadVersion(3) -> false
//! call isBadVersion(5) -> true
//! call isBadVersion(4) -> true
//! Then 4 is the first bad version.
//! Example 2:
//!
//! Input: n = 1, bad = 1
//! Output: 1
//!
//!
//! Constraints:
//!
//! 1 <= bad <= n <= 231 - 1

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)
use std::cmp::min;

pub struct Solution {}

impl Solution {
	/// is just dummy method
	pub fn isBadVersion(&self, n: i32) -> bool {
		true
	}
	pub fn first_bad_version(&self, n: i32) -> i32 {
		let mut left = 1;
		let mut right = n;
		let mut fbv = n;
		while left <= right {
			let pivot = left + (right - left) / 2;
			if self.isBadVersion(pivot) {
				fbv = min(fbv, pivot);
				right = pivot - 1;
			} else {
				left = pivot + 1;
			}
		}
		fbv
	}

	pub fn first_bad_version_best_prac(&self, n: i32) -> i32 {
		//  binary search
		let mut lo: i32 = 1;
		let mut hi: i32 = n;

		while lo <= hi {
			let mid: i32 = lo + (hi - lo) / 2;
			if self.isBadVersion(mid) {
				hi = mid - 1;
			} else {
				lo = mid + 1;
			}
		}
		return lo;
	}
}
