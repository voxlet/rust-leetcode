pub struct Solution {}

fn brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, a) in nums.iter().enumerate() {
        for (j, b) in nums.iter().skip(i + 1).enumerate() {
            if a + b == target {
                return vec![i as i32, (j + i + 1) as i32];
            }
        }
    }
    vec![]
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        brute_force(nums, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(brute_force(vec![1, 2, 3], 5), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![1, 2, 3], 5), vec![1, 2]);
    }
}
