use std::ops::{Add, AddAssign, Div, Mul, Rem, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T: Mul<Output = T>> Vector<T> {
    pub fn to_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    // fn mul(self, val: T) -> Vector<T> {
    //     Vector {
    //         x: self.x * val,
    //         y: self.y * val,
    //     }
    // }
}

// impl<T: Rem<Output = T>> Rem<T> for Vector<T> {
//     type Output = Self;
//
//     fn rem(self, modulus: T) -> Self::Output {
//         Self {
//             x: self.x % modulus,
//             y: self.y % modulus,
//         }
//     }
// }

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

// impl<T: Mul<Output = T>> Mul<T> for Vector<T> {
//     type Output = Self;
//
//     fn mul(self, rhs: T) -> Self::Output {
//         Self {
//             x: self.x * rhs,
//             y: self.y * rhs,
//         }
//     }
// }

impl<T: Mul<Output = T>> Mul for Vector<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T: Div<Output = T>> Div for Vector<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

// impl<T: Div<Output = T>> Div<T> for Vector<T> {
//     type Output = Self;
//
//     fn div(self, rhs: T) -> Self::Output {
//         Self {
//             x: self.x / rhs,
//             y: self.y / rhs,
//         }
//     }
// }

impl Vector<f32> {
    pub fn round_i_int(self) -> Vector<i16> {
        Vector {
            x: self.x.round() as i16,
            y: self.y.round() as i16,
        }
    }
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

    pub fn round(self) -> Vector<f32> {
        Vector {
            x: self.x.round(),
            y: self.y.round(),
        }
    }
    pub fn ceil(self) -> Vector<f32> {
        Vector {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    pub fn floor(self) -> Vector<f32> {
        Vector {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }
}

impl Vector<u16> {
    pub fn to_i16_tuple(self) -> (i16, i16) {
        (self.x as i16, self.y as i16)
    }

    pub fn to_f32_tuple(self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }
}
