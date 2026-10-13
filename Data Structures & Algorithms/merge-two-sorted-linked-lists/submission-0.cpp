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
    ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) {
        ListNode* head;
        ListNode* p;
        if(l1 == nullptr) {
            return l2;
        } else if(l2 == nullptr) {
            return l1;
        }
        if(l1->val < l2->val) {
            head = l1;
            p = l1;
            l1 = l1->next;
        } else {
            head = l2;
            p = l2;
            l2 = l2->next;
        }
        
        while(l1 != nullptr || l2 !=nullptr) {
            if(l1 == nullptr) {
                p->next = l2;
                break;
            } else if(l2 == nullptr) {
                p->next = l1;
                break;
            } else if(l1->val < l2->val) {
                p->next = l1;
                p = p->next;
                l1 = l1->next;
            } else {
                p->next = l2;
                p = p->next;
                l2 = l2->next;
            }
        }
        return head;
    }
};
