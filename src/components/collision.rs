use specs::{Component, VecStorage};

#[derive(Debug, Clone, Copy)]
pub enum OnCollideType {
    Delete,
    Block,
}

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Collide {
    pub on_collide: OnCollideType,
}

impl Collide {
    pub fn new(on_collide: OnCollideType) -> Self {
        Self { on_collide }
    }
}
