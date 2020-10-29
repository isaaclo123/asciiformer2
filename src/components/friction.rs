use specs::{Component, VecStorage};

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Friction {
    pub friction: f32,
}

impl Friction {
    pub fn new(friction: f32) -> Self {
        Self { friction }
    }
}
