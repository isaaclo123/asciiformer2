use specs::{Component, NullStorage};

#[derive(Component, Debug, Default, Clone, Copy)]
#[storage(NullStorage)]
pub struct KeyboardControlled;
