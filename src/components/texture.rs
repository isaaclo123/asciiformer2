use crate::components::Position;
use euclid::default::Vector2D;
use specs::{Component, VecStorage};

pub type Tile = &'static [&'static [char]];

#[non_exhaustive]
pub struct PlayerTiles;

impl PlayerTiles {
    pub const NO_EXTEND: Tile = &[&['█']];
    pub const Y_EXTEND: Tile = &[&['▄'], &['▀']];
    pub const X_Y_EXTEND: Tile = &[&['▗', '▖'], &['▝', '▘']];
    pub const X_EXTEND: Tile = &[&['▐', '▌']];
}

#[non_exhaustive]
pub struct WallTiles;
impl WallTiles {
    pub const WALL: Tile = &[&['█']];
}

#[non_exhaustive]
pub struct AirTiles;
impl AirTiles {
    pub const AIR: Tile = &[&[' ']];
}

#[non_exhaustive]
pub struct BulletTiles;
impl BulletTiles {
    pub const BOT_LEFT: Tile = &[&['▖']];
    pub const BOT_RIGHT: Tile = &[&['▗']];
    pub const TOP_LEFT: Tile = &[&['▘']];
    pub const TOP_RIGHT: Tile = &[&['▝']];
    pub const MID: Tile = &[&['▮']];
}

pub trait TextureConfig: Sync {
    // fn new(&self) -> Self where Self::Sized;
    fn get_texture(&self, position: &Position) -> Tile;
}

pub struct BulletTextures;
impl TextureConfig for BulletTextures {
    fn get_texture(&self, position: &Position) -> Tile {
        let Vector2D { x, y, .. } = position.0;
        let x = x.fract();
        let y = y.fract();

        if x > 0.4 && x < 0.6 && y > 0.4 && y < 0.6 {
            return BulletTiles::MID;
        }

        match (y < 0.5, x < 0.5) {
            // is_top, is_left
            (true, true) => BulletTiles::TOP_LEFT,
            (false, true) => BulletTiles::BOT_LEFT,
            (true, false) => BulletTiles::TOP_RIGHT,
            (false, false) => BulletTiles::BOT_RIGHT,
        }
    }
}

pub struct PlayerTextures;
impl TextureConfig for PlayerTextures {
    fn get_texture(&self, position: &Position) -> Tile {
        let Vector2D { x, y, .. } = position.0;
        let x = x.fract();
        let y = y.fract();

        match (x < 0.5, y < 0.5) {
            (true, true) => PlayerTiles::NO_EXTEND,
            (true, false) => PlayerTiles::Y_EXTEND,
            (false, true) => PlayerTiles::X_EXTEND,
            (false, false) => PlayerTiles::X_Y_EXTEND,
        }
    }
}

pub struct WallTextures;
impl TextureConfig for WallTextures {
    fn get_texture(&self, _: &Position) -> Tile {
        WallTiles::WALL
    }
}

#[derive(Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Texture {
    config: &'static dyn TextureConfig,
}

impl Texture {
    pub fn new(config: &'static dyn TextureConfig) -> Self {
        Self { config }
    }

    pub fn get_texture(&self, position: &Position) -> Tile {
        self.config.get_texture(position)
    }
}
