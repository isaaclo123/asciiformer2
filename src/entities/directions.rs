use crate::vectors::Vector;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    To(Vector<f32>),
    None,
}
