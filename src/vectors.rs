use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector<T> {
    pub fn to_tuple(self) -> (T, T) {
        (self.x, self.y)
    }
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

impl Vector<f32> {
    pub fn round_int(self) -> Vector<u16> {
        Vector {
            x: self.x.round() as u16,
            y: self.y.round() as u16,
        }
    }

    pub fn ceil_int(self) -> Vector<u16> {
        Vector {
            x: self.x.ceil() as u16,
            y: self.y.ceil() as u16,
        }
    }

    pub fn floor_int(self) -> Vector<u16> {
        Vector {
            x: self.x.floor() as u16,
            y: self.y.floor() as u16,
        }
    }
}
