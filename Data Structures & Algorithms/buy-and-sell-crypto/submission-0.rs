impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut buy_price = i32::MAX;
        for p in prices.into_iter() {
            buy_price = buy_price.min(p);
            max = max.max(p - buy_price);
        }
        max
    }
}
