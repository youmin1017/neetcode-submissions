use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for i in 0..nums.len() {
            if let Some(j) = map.get(&nums[i]) {
                return vec![(*j).try_into().unwrap(), i.try_into().unwrap()];
            } else {
                map.insert(target-nums[i], i);
            }
        }

        vec![]
    }
}

