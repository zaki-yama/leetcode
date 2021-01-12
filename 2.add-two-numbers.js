/*
 * @lc app=leetcode id=2 lang=javascript
 *
 * [2] Add Two Numbers
 */


function ListNode(val) {
    this.val = val;
    this.next = null;
}
// @lc code=start
/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */
/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
var addTwoNumbers = function(l1, l2) {
    let l1temp = l1;
    let l2temp = l2;

    const ans = new ListNode();
    let ansTemp = ans;
    let carry = false;
    while (l1temp || l2temp) {
      const sum = (l1temp?.val || 0) + (l2temp?.val || 0) + (carry ? 1 : 0);
      const val = sum % 10;
      console.log(`${l1temp?.val} + ${l2temp?.val} = ${val}`);
      carry = sum >= 10;
      const node = new ListNode(val);
      ansTemp.next = node;
      ansTemp = node;

      console.log('ansTemp', ansTemp);
      l1temp = l1temp?.next;
      l2temp = l2temp?.next;
    }
    if (carry) {
      ansTemp.next = new ListNode(1);
    }

    return ans.next;
};
// @lc code=end

function fromArray(arr) {

  let prev = new ListNode();
  const ans = prev;
  arr.forEach(val => {
    console.l
    const node = new ListNode(val);
    prev.next = node;
    prev = node;
  });
  return ans.next;
}

// const l1 = fromArray([2, 4, 3]);
// const l2 = fromArray([5, 6, 4]);
const l1 = fromArray([9,9,9,9,9,9,9]);
const l2 = fromArray([9,9,9,9]);

console.log(addTwoNumbers(l1, l2));
