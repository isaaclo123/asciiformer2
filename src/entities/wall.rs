use super::Entity;
use crate::textures::{Texture, WallTextures};
use crate::vectors::Vector;

/* Walls */

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

impl<'a> Entity<'a> for Wall {
    fn get_texture(&self) -> Texture {
        return WallTextures::WALL;
    }

    fn get_point(&self) -> Vector<u16> {
        self.point.floor_int()
    }

    fn to_string(&self) -> &'a str {
        "Wall"
    }
}
