use crate::consts::{EntityType, TEXTURE_MAP};
use crate::vectors::Vector;
use std::io::Write;
use std::ops::{Add, Mul};
use termion::{clear, color, cursor};

pub trait Entity<'a> {
    fn update(&mut self);
    fn get_x(&mut self) -> u16;
    fn get_y(&mut self) -> u16;
    fn get_texture(&mut self) -> char;
    fn draw(&mut self, stdout: &mut impl Write);
    fn collide(&mut self, entity: &impl Entity<'a>);
    fn to_string(&mut self) -> &'a str;
}

pub struct Player<'a> {
    point: Vector<u16>,
    velocity: Vector<u16>,
    name: &'a str,
}

impl<'a> Entity<'a> for Player<'a> {
    fn update(&mut self) {}
    fn get_x(&mut self) -> u16 {
        self.point.x
    }
    fn get_y(&mut self) -> u16 {
        self.point.y
    }
    fn draw(&mut self, stdout: &mut impl Write) {
        write!(
            stdout,
            "{}X",
            cursor::Goto(self.point.x + 1, self.point.y + 1)
        )
        .unwrap();
    }
    fn get_texture(&mut self) -> char {
        *TEXTURE_MAP.get(EntityType::PLAYER).unwrap()
    }
    fn collide(&mut self, entity: &impl Entity<'a>) {}
    fn to_string(&mut self) -> &'a str {
        self.name
    }
}
