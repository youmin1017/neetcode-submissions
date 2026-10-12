class Solution {
public:
    int largestRectangleArea(vector<int>& heights) {
        heights.push_back(0);
        int max_area = 0;
        vector<int> s;

        for(int i = 0; i < heights.size(); ++i) {
            while(!s.empty() && heights[i] < heights[s.back()]) {
                int last = s.back();
                s.pop_back();
                int margin_l = -1;
                if (!s.empty()) {
                    margin_l = s.back();
                }
                
                int area = (i - margin_l - 1) * heights[last];
                max_area = max(max_area, area);
           }
            s.push_back(i);
        }
        return max_area;
    }
};
 