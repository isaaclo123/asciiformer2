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

#[non_exhaustive]
pub struct AirTextures;
impl AirTextures {
    pub const AIR: Texture = &[&[' ']];
}

#[non_exhaustive]
pub struct BulletTextures;
impl BulletTextures {
    pub const BOT_LEFT: Texture = &[&['▖']];
    pub const BOT_RIGHT: Texture = &[&['▗']];
    pub const TOP_LEFT: Texture = &[&['▘']];
    pub const TOP_RIGHT: Texture = &[&['▝']];
    pub const MID: Texture = &[&['▮']];
}
