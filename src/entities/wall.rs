use super::Entity;
use crate::textures::{Texture, WallTextures};
use crate::vectors::Vector;

/* Walls */

#[derive(Clone, Copy)]
pub struct Wall {
    pub point: Vector<f32>,
}

impl Wall {
    pub fn new(x: u16, y: u16) -> Wall {
        return Wall {
            point: Vector {
                x: x as f32,
                y: y as f32,
            },
        };
    }
}

impl Entity for Wall {
    fn get_id(&self) -> Option<i16> {
        Some(-1)
    }

    fn get_texture(&self) -> Texture {
        return WallTextures::WALL;
    }

    fn get_point(&self) -> Vector<f32> {
        self.point
    }

    fn to_string(&self) -> &str {
        "Wall"
    }
}
