use crate::vectors::Vector;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    To(Vector<f32>),
    None,
}
unsafe impl Send for Direction {}
unsafe impl Sync for Direction {}
