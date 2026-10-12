class Solution {
public:
    int largestRectangleArea(vector<int>& heights) {
        int n = heights.size();
        long long max_area = 0;
        vector<int> stk;  // 存索引,對應高度由底到頂遞增

        for (int i = 0; i <= n; ++i) {
            int cur = (i == n) ? 0 : heights[i];  // i == n 時當作哨兵 0
            while (!stk.empty() && cur < heights[stk.back()]) {
                int height = heights[stk.back()];
                stk.pop_back();
                int left = stk.empty() ? -1 : stk.back();
                max_area = max(max_area, 1LL * height * (i - left - 1));
            }
            stk.push_back(i);
        }
        return (int)max_area;
    }
};