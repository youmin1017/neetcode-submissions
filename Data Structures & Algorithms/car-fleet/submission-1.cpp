class Solution {
public:
    int carFleet(int target, vector<int>& position, vector<int>& speed) {
        vector<pair<int, int>> cars;
        for(size_t i = 0; i < position.size(); i++) {
            cars.emplace_back(position[i], speed[i]);
        }
        sort(cars.rbegin(), cars.rend());
        vector<double> times;
        for(auto &c : cars) {
            times.push_back((double)(target-c.first) / c.second);
            if(times.size() >= 2 && times.back() <= times[times.size() - 2]) {
                times.pop_back();
            }
        }
        return times.size();
    }
};

// stack
// 5
// 3 pop
// 6 push: cause it take longer than latest