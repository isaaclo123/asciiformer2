use super::Entity;
use crate::textures::{Texture, WallTextures};
use crate::vectors::Vector;
use termion::color;

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

    fn get_color(&self) -> Option<&'a dyn color::Color> {
        None
    }

    fn get_point(&self) -> Vector<u16> {
        self.point.floor_int()
    }

    fn collide(&mut self, _entity: &'a mut impl Entity<'a>) {}

    fn to_string(&self) -> &'a str {
        "Wall"
    }

    fn update(&mut self) {}
}
