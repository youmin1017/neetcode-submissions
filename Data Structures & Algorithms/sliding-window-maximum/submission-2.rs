impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut current_max = i32::MIN;
        let mut max_index = 0;
        let k = k as usize;

        for (i, n) in nums[0..k].iter().enumerate() {
            if *n >= current_max {
                current_max = *n;
                max_index = i;
            }
        }

        let mut res = vec![current_max];
        for i in 1..=nums.len() - k {
            if i > max_index {
                current_max = i32::MIN;
                for j in i..i + k {
                    if nums[j] > current_max {
                        current_max = nums[j];
                        max_index = j;
                    }
                }
                res.push(current_max);
            } else if nums[i + k - 1] >= current_max {
                current_max = nums[i + k - 1];
                max_index = i + k - 1;
                res.push(current_max);
            } else {
                // current max in window and current max > nums[i]
                res.push(current_max);
            }
        }

        res
    }
}



