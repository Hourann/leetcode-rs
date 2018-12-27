// https://leetcode.com/problems/container-with-most-water/
pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, height.len() - 1);
        let mut max_area = 0;
        while i < j {
            if height[i] < height[j] {
                max_area = max_area.max(height[i] * (j - i) as i32);
                i += 1;
            } else {
                max_area = max_area.max(height[j] * (j - i) as i32);
                j -= 1;
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn p11() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
