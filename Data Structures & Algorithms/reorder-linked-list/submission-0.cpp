class Solution {
public:
    void reorderList(ListNode* head) {
        if (!head || !head->next) return;

        ListNode* slow = head;
        ListNode* fast = head->next;

        while (fast != nullptr && fast->next != nullptr) {
            slow = slow->next;
            fast = fast->next->next;
        }

        ListNode* s = reverse(slow->next);  // 反轉後半段
        slow->next = nullptr;               // 截斷前半段

        ListNode* p = head;                 // 初始化
        while (s != nullptr) {
            ListNode* p_next = p->next;
            ListNode* s_next = s->next;
            p->next = s;
            s->next = p_next;
            p = p_next;
            s = s_next;
        }
    }

    ListNode* reverse(ListNode* head) {
        ListNode* prev = nullptr;
        while (head != nullptr) {
            ListNode* temp = head->next;
            head->next = prev;
            prev = head;
            head = temp;
        }
        return prev;
    }
};