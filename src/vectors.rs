use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

// Notice that the implementation uses the associated type `Output`.
impl<T: Add<Output = T>> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Vector<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
