//! # Add Two numbers / Medium(leet code)
//! [문제 링크](https://leetcode.com/problems/add-two-numbers/)
//!
//! You are given two **non-empty** linked lists representing two non-negative integers. The digits are stored in **reverse order**, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
//!
//! You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg)
//!
//! ```
//! Input: l1 = [2,4,3], l2 = [5,6,4]
//! Output: [7,0,8]
//! Explanation: 342 + 465 = 807.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: l1 = [0], l2 = [0]
//! Output: [0]
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
//! Output: [8,9,9,9,0,0,0,1]
//!
//! ```
//!
//! **Constraints:**
//!
//! -   The number of nodes in each linked list is in the range `[1, 100]`.
//! -   `0 <= Node.val <= 9`
//! -   It is guaranteed that the list represents a number that does not have leading zeros.
//!
//! # 문제 풀이 구상
//! 1. 상수로 만들어서 계산하고 쪼개면 간단하겠다.
//! 	제한 조건 노드 max 100 개, 100자리 정수면, 2^330, 즉 u128 로도 모자람. 폐기

#[cfg(test)]
mod test {
	use super::*;
	// #[test]
	// fn case_1() {
	// 	let mut l1 = Box::new(ListNode::new(5));
	// 	l1.next = Some(Box::new(ListNode::new(6)));
	// 	l1.next.and_then(|this| {this.next = Some(Box::new(ListNode::new(4)))})

	// 	let mut l2 = Box::new(ListNode::new(5));
	// 	l2.next = Some(Box::new(ListNode::new(6)));
	// 	l2.next.unwrap().next = Some(Box::new(ListNode::new(4)));

	// 	let mut output = Box::new(ListNode::new(7));
	// 	output.next = Some(Box::new(ListNode::new(0)));
	// 	output.next.unwrap().next = Some(Box::new(ListNode::new(8)));

	// 	assert_eq!(output, add_two_numbers(Some(l1), Some(l2)).unwrap());
	// }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
	pub val: i32,
	pub next: Option<Box<ListNode>>,
}

impl ListNode {
	#[inline]
	fn new(val: i32) -> Self {
		ListNode { next: None, val }
	}

	fn push(&mut self, val: i32) {
		match &mut self.next {
			None => self.next = Some(Box::new(ListNode::new(val))),
			Some(n) => self.push(val),
		}
	}
}
pub struct Solution {}

impl Solution {
	//! # 결과
	//! Runtime: 8 ms, faster than 34.72% of Rust online submissions for Add Two Numbers.
	//! Memory Usage: 2.3 MB, less than 32.79% of Rust online submissions for Add Two Numbers.
	pub fn add_two_numbers(
		l1: Option<Box<ListNode>>,
		l2: Option<Box<ListNode>>,
	) -> Option<Box<ListNode>> {
		return Solution::calculator(l1, l2, 0);
	}

	pub fn calculator(
		node1: Option<Box<ListNode>>,
		node2: Option<Box<ListNode>>,
		increment: i32,
	) -> Option<Box<ListNode>> {
		match (node1, node2) {
			(None, None) => {
				if increment > 0 {
					return Some(Box::new(ListNode {
						next: None,
						val: increment,
					}));
				}
				return None;
			}
			(None, Some(a)) | (Some(a), None) => {
				let sum = a.val + increment;
				return Some(Box::new(ListNode {
					next: Solution::calculator(None, a.next, sum / 10),
					val: sum % 10,
				}));
			}
			(Some(a), Some(b)) => {
				let sum = a.val + b.val + increment;
				return Some(Box::new(ListNode {
					next: Solution::calculator(a.next, b.next, sum / 10),
					val: sum % 10,
				}));
			}
		}
	}
}
