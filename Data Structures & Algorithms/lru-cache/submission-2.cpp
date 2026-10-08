class LRUCache {
public:
    unordered_map<int, pair<int, list<int>::iterator>> cache;
    list<int> order;
    int limit;
    
    LRUCache(int capacity) {
        limit = capacity;
    }
    
    /**get
      * 1. cached
      */
    int get(int key) {
       if(cache.find(key) != cache.end()) {
            auto v = cache[key];
            order.erase(v.second);
            order.push_back(key);
            cache[key] = {v.first, --order.end()};
            return v.first;
       } 
       return -1;
    }
    
    /**put
      * 1. hitting
      * 2. reach size limiting
      * 3. non of above
      */
    void put(int key, int value) {
        if (cache.find(key) != cache.end()) {
            auto v = cache[key];
            order.erase(v.second);
        }
        else if (cache.size() == limit) {
           int lru = order.front();
            order.pop_front();
            cache.erase(lru);
        }
        order.push_back(key);
        cache[key] = {value, --order.end()};
    }
};
