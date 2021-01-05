/*
 * @lc app=leetcode id=82 lang=typescript
 *
 * [82] Remove Duplicates from Sorted List II
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */
function deleteDuplicates(head: ListNode | null): ListNode | null {

  if (head === null) {
    return null;
  }

  const fakeHead = new ListNode();
  fakeHead.next = head;
  let pre = fakeHead;
  let cursor = head;

  while (cursor) {
    while (cursor.next && cursor.val === cursor.next.val) {
      cursor = cursor.next;
    }

    if (pre.next === cursor) {
      pre = pre.next;
    }
    else {
      pre.next = cursor.next;
    }
    cursor = cursor.next;
  }
  return fakeHead.next;
};
// @lc code=end

