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
    // if it's a beginning of duplicates sublist
    // skip all duplicates
    if (cursor.next && cursor.val === cursor.next.val) {
      while (cursor.next && cursor.val === cursor.next.val) {
        cursor = cursor.next;
      }
      pre.next = cursor.next;

    // otherwise, move predecessor
    } else {
      pre = pre.next;
    }

    cursor = cursor.next;
  }
  return fakeHead.next;
};
// @lc code=end

