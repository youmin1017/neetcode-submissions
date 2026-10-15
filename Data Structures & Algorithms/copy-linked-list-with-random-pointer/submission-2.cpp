/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* next;
    Node* random;
    
    Node(int _val) {
        val = _val;
        next = NULL;
        random = NULL;
    }
};
*/

class Solution {
public:
    Node* copyRandomList(Node* head) {
        if (head == nullptr) {
            return nullptr;
        }
        vector<Node*> v;
        Node* p = head;

        unordered_map<Node*, int> p2idx {{nullptr, -1}};
        int i = 0;
        
        while(p != nullptr){ 
            Node* n = new Node(p->val);
            v.push_back(n);
            p2idx[p] = i;
            i += 1;
            p = p->next;
        }

        vector<int> rands;
        p = head;
        while(p != nullptr) {
            int idx = p2idx[p->random];
            rands.push_back(idx);
            p = p->next;
        }

        for(int i=1; i < v.size(); ++i) {
            v[i-1]->next = v[i];
            v[i]->random = rands[i] == -1 ? nullptr : v[rands[i]];
        }
        v[0]->random = rands[0] == -1 ? nullptr : v[rands[0]];
        return v[0];
    }
};
