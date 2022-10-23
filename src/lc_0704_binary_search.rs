use std::cmp::Ordering::*;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    match bin_helper(&nums, target) {
        Some(pos) => pos as i32,
        None => -1
    }
}

pub fn bin_helper(nums: &[i32], target: i32) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }
    let mid = nums.len() /2;

    match target.cmp(&nums[mid]) {
        Equal => Some(mid),
        Less => bin_helper(&nums[..mid], target),
        Greater => match bin_helper(&nums[mid + 1..], target) {
            Some(pos) => Some(mid + 1 + pos),
            None => None
        },
    }
}

#[cfg(test)]
mod test {
    use crate::lc_0704_binary_search::search;

    #[test]
    fn ex1() {
        let nums = vec![-1,0,3,5,9,12];
        assert_eq!(search(nums,9),4);
    }

    #[test]
    fn ex2() {
        let nums = vec![-1,0,3,5,9,12];
        assert_eq!(search(nums,2),-1);
    }

    #[test]
    fn ex3() {
        let nums = vec![-1,0,3,9,12];
        assert_eq!(search(nums,12),4);
    }
}