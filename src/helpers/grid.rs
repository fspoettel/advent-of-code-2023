#[derive(Debug, Clone)]
pub struct Point<T: Clone = i32>(pub T, pub T);

impl Point<i32> {
    pub fn distance(&self, other: &Point<i32>) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

impl Point<i64> {
    pub fn distance(&self, other: &Point<i64>) -> u64 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}
