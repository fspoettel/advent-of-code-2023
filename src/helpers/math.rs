pub fn least_common_multiple(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = least_common_multiple(&nums[1..]);
    a * b / greatest_common_divisor(a, b)
}

fn greatest_common_divisor(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    greatest_common_divisor(b, a % b)
}
