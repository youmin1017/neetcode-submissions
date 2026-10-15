/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

class Solution {
public:
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode* rev = reverseList(head);
        ListNode* p = rev;
        ListNode* prev = rev;
        
        if(n == 1) {
            return reverseList(rev->next);
        }

        for(int i=1; i < n; ++i) {
            prev = p;
            p = p->next;
        }
        prev->next = p->next;
        return reverseList(rev);
    }

    ListNode* reverseList(ListNode* head) {
        ListNode* p = head;
        ListNode* prev = nullptr;
        while(p != nullptr) {
            auto temp = p->next;
            p->next = prev;
            prev = p;
            p = temp;
        }
        return prev;
    }
};
