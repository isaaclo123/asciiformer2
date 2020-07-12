pub type Texture = &'static [&'static [char]];

#[non_exhaustive]
pub struct PlayerTextures;

impl PlayerTextures {
    pub const NO_EXTEND: Texture = &[&['█']];

    pub const Y_EXTEND: Texture = &[&['▄'], &['▀']];

    pub const X_Y_EXTEND: Texture = &[&['▗', '▖'], &['▝', '▘']];

    pub const X_EXTEND: Texture = &[&['▐', '▌']];
}

#[non_exhaustive]
pub struct WallTextures;

impl WallTextures {
    pub const WALL: Texture = &[&['█']];
}
