use crate::components::{Color, Position, Texture};
use crate::io::{get_stdout, STDOUT};
use specs::{Join, ReadStorage, System};
use std::io::Write;
use vector2math::*;

use termion::{clear, color, cursor};

pub struct Renderer {
    origin: (u16, u16),
    to_clear: Vec<Position>,
}

impl Renderer {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            origin: (x, y),
            to_clear: vec![],
        }
    }
}

impl<'a> System<'a> for Renderer {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Color>,
        ReadStorage<'a, Texture>,
    );

    fn run(&mut self, (position, color, texture): Self::SystemData) {
        write!(get_stdout(), "{}", clear::All).unwrap();

        for (pos, color, tex) in (&position, &color, &texture).join() {
            let (floor_x, floor_y) = pos.floor();
            let draw_pt = self
                .origin
                .add((floor_x as u16, floor_y as u16))
                .add((1, 1));

            let texture = tex.get_texture(pos);

            for y in 0..texture.len() {
                for x in 0..texture[y].len() {
                    writeln!(
                        get_stdout(),
                        "{goto}{color}{sym}",
                        goto = cursor::Goto(draw_pt.x() + x as u16, draw_pt.y() + y as u16),
                        color = color::Fg(color.get_color()),
                        // sym = sym
                        sym = texture[y][x]
                    )
                    .unwrap();
                }
            }
        }
    }
}
