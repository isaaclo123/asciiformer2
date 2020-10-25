use specs::{Component, NullStorage};

#[derive(Component, Debug, Default, Clone, COpy)]
#[storage(NullStorage)]
pub struct KeyboardControlled;
