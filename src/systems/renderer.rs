use crate::components::{Color, Position, Texture};
use crate::io::get_stdout;
use crate::resources::{Map, WALL};
use euclid::default::Vector2D;
use specs::{Join, Read, ReadStorage, System};
use std::io::Write;

use termion::{clear, color, cursor};

pub struct Renderer {
    draw_map: bool,
    origin: Vector2D<u16>,
    to_clear: Vec<Position>,
}

impl Renderer {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            draw_map: true,
            origin: Vector2D::new(x, y),
            to_clear: vec![],
        }
    }

    fn draw_map(&self, map: &Map) -> () {
        for y in 0..map.height {
            for x in 0..map.width {
                if map.wall_get_unchecked(x as i16, y as i16) {
                    let draw_pt = self.origin + Vector2D::new(x as u16 + 1, y as u16 + 1);
                    writeln!(
                        get_stdout(),
                        "{goto}{texture}",
                        goto = cursor::Goto(draw_pt.x, draw_pt.y),
                        texture = WALL
                    )
                    .unwrap();
                }
            }
        }
    }
}

impl<'a> System<'a> for Renderer {
    type SystemData = (
        Read<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Color>,
        ReadStorage<'a, Texture>,
    );

    fn run(&mut self, (map, position, color, texture): Self::SystemData) {
        write!(get_stdout(), "{}", clear::All).unwrap();

        if self.draw_map {
            self.draw_map(&map);
            self.draw_map = false;
        }

        for (pos, color, tex) in (&position, (&color).maybe(), &texture).join() {
            let Vector2D { x, y, .. } = pos.0.floor();

            let draw_pt = self.origin + Vector2D::new(x as u16 + 1, y as u16 + 1);

            let color = if let Some(c) = color {
                c.get_color()
            } else {
                &color::Reset
            };
            let texture = tex.get_texture(pos);

            for y in 0..texture.len() {
                for x in 0..texture[y].len() {
                    writeln!(
                        get_stdout(),
                        "{goto}{color}{sym}",
                        goto = cursor::Goto(draw_pt.x + x as u16, draw_pt.y + y as u16),
                        color = color::Fg(color),
                        sym = texture[y][x]
                    )
                    .unwrap();
                }
            }
        }
    }
}
