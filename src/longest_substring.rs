//! # Longest Substring Without Repeating Characters / medium(leet code)
//!
//! [문제 링크](https://leetcode.com/problems/longest-substring-without-repeating-characters/)
//!
//! Given a string `s`, find the length of the **longest substring** without repeating characters.
//!
//! **Example 1:**
//!
//! ```
//! Input: s = "abcabcbb"
//! Output: 3
//! Explanation: The answer is "abc", with the length of 3.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: s = "bbbbb"
//! Output: 1
//! Explanation: The answer is "b", with the length of 1.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: s = "pwwkew"
//! Output: 3
//! Explanation: The answer is "wke", with the length of 3.
//! Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//!
//! ```
//!
//! **Constraints:**
//!
//! -   `0 <= s.length <= 5 * 104`
//! -   `s` consists of English letters, digits, symbols and spaces.

use std::cmp::max;
use std::collections::{HashMap, VecDeque};

#[cfg(test)]
mod test {
	use super::Solution;
	#[test]
	fn case_1() {
		assert_eq!(
			Solution::length_of_longest_substring("abcabcbb".to_owned()),
			3
		)
	}

	#[test]
	fn case_1_best_prac() {
		assert_eq!(
			Solution::length_of_longest_substring_best_prac("abcabcbb".to_owned()),
			3
		)
	}
	#[test]
	fn case_2() {
		assert_eq!(Solution::length_of_longest_substring("bbbbb".to_owned()), 1)
	}

	#[test]
	fn case_3() {
		assert_eq!(
			Solution::length_of_longest_substring("pwwkew".to_owned()),
			3
		)
	}
}

pub struct Solution {}

impl Solution {
	/// # 결과
	/// Runtime: 2 ms, faster than 82.41% of Rust online submissions for Longest Substring Without Repeating Characters.
	///
	/// Memory Usage: 2.1 MB, less than 83.12% of Rust online submissions for Longest Substring Without Repeating Characters.
	pub fn length_of_longest_substring(s: String) -> i32 {
		let mut bucket: VecDeque<char> = VecDeque::new();
		let mut max_len = 0;
		for c in s.chars() {
			if bucket.contains(&c) {
				if max_len < bucket.len() {
					max_len = bucket.len();
				}
				loop {
					let val = bucket.pop_front().unwrap();
					if c == val {
						break;
					}
				}
			}
			bucket.push_back(c);
		}
		if max_len < bucket.len() {
			max_len = bucket.len();
		}
		max_len as i32
	}

	pub fn length_of_longest_substring_best_prac(s: String) -> i32 {
		let mut m = HashMap::new();
		let mut ans = 0;
		let mut before = -1;
		let mut current = 0;
		for c in s.chars() {
			if let Some(last) = m.insert(c, current) {
				before = max(before, last);
			}
			ans = max(ans, current - before);
			current += 1;
		}
		ans
	}
}
