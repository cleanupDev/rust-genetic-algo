fn main() {
    println!("Hello, world!");
}


fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            left = mid + 1;
        } else if nums[mid] > target {
            right = mid - 1;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }
}
