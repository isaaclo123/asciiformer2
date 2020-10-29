use specs::{Component, VecStorage};

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Gravity {
    pub gravity: f32,
}

impl Gravity {
    pub fn new(gravity: f32) -> Self {
        Self { gravity }
    }
}
