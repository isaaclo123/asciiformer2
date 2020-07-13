use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

impl<T: AddAssign + Add<Output = T> + Copy> AddAssign for Vector<T> {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl<T: Sub<Output = T>> Sub for Vector<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
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
