use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in nums.into_iter() {
            map.entry(n).and_modify(|n| *n += 1).or_insert(0);
        }

        let mut sorted: Vec<(i32, i32)> = map.into_iter().collect();
        sorted.sort_by_key(|v| -v.1);

        sorted.into_iter().map(|v| v.0).take(k as usize).collect()
    }
}