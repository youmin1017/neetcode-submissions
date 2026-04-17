use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map: HashMap<char, i16> = HashMap::new();

        if s.len() != t.len() {
            return false;
        }

        for (cs, ct) in s.chars().zip(t.chars()) {
            map.entry(cs).and_modify(|v| *v += 1).or_insert(1);
            map.entry(ct).and_modify(|v| *v -= 1).or_insert(-1);
        }
        map.iter().all(|(_, v)| v.eq(&0))
    }
}