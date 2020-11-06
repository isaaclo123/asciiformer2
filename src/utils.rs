use euclid::default::Vector2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    To(Vector2D<i16>),
}
