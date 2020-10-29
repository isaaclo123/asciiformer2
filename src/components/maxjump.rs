use specs::{Component, VecStorage};

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct MaxJump {
    pub max_jump: u16,
    pub jump: u16,
}

impl MaxJump {
    pub fn new(max_jump: u16) -> Self {
        Self { jump: 0, max_jump }
    }
}
