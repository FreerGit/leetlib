

pub fn max_profit(prices: Vec<i32>) -> i32 {
    use std::cmp::{min, max};
    let mut mx = 0;
    let mut b = std::i32::MAX;
    prices.into_iter().for_each(|value| {
        mx = max(mx, value - b);
        b = min(b, value);
    });
    mx
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(max_profit(vec![7,1,5,3,6,4]), 5);
    }
    #[test]
    fn ex2() {
        assert_eq!(max_profit(vec![7,6,4,3,1]),0);
    }
}
