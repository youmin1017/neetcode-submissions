use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut l = 0;
        let mut max_freq = 0;
        let mut freq_map: HashMap<char, i32> = HashMap::new();
        let mut max_len = 0;

        for (r, c) in s.iter().enumerate() {
            let v = freq_map.entry(*c).or_insert(0);
            *v += 1;
            max_freq = max_freq.max(*v);

            // means insuffcient
            while  (r - l + 1 - max_freq as usize) > k as usize {
                freq_map.entry(s[l]).and_modify(|v| *v -= 1);
                l += 1;
            }
            max_len = max_len.max((r-l+1) as i32);
        }
        max_len
    }
}
