use crate::components::{Color, Position, Texture};
use crate::io::get_stdout;
use crate::resources::{Map, AIR, WALL};
use euclid::default::Vector2D;
use specs::{Join, Read, ReadStorage, System};
use std::io::Write;

use termion::{clear, color, cursor};

pub struct Renderer {
    draw_map: bool,
    origin: Vector2D<u16>,
    to_clear: Vec<Vector2D<u16>>,
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
                    write!(
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

    fn clear_prev(&mut self, map: &Map) -> () {
        let draw_pt = self.origin + Vector2D::new(1, 1);

        for pt in self.to_clear.iter() {
            let clear_pt = draw_pt + *pt;

            let sym = if map.wall_get(clear_pt.x as i16, clear_pt.y as i16) {
                WALL
            } else {
                AIR
            };

            write!(
                get_stdout(),
                "{goto}{sym}",
                goto = cursor::Goto(clear_pt.x, clear_pt.y),
                sym = sym
            )
            .unwrap();
        }

        self.to_clear.clear();
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
        if self.draw_map {
            write!(get_stdout(), "{}{}", clear::All, cursor::Hide).unwrap();
            self.draw_map(&map);
            self.draw_map = false;
        }

        self.clear_prev(&map);

        for (pos, color, tex) in (&position, (&color).maybe(), &texture).join() {
            let start_pt = pos.0.floor().cast::<u16>();

            let color = if let Some(c) = color {
                c.get_color()
            } else {
                &color::Reset
            };
            let texture = tex.get_texture(pos);

            for y in 0..texture.len() {
                for x in 0..texture[y].len() {
                    let texture_pt = start_pt + Vector2D::new(x as u16, y as u16);

                    let draw_pt = self.origin + Vector2D::new(1, 1) + texture_pt;

                    write!(
                        get_stdout(),
                        "{goto}{color}{sym}",
                        goto = cursor::Goto(draw_pt.x, draw_pt.y),
                        color = color::Fg(color),
                        sym = texture[y][x]
                    )
                    .unwrap();

                    self.to_clear.push(texture_pt);
                }
            }
        }

        get_stdout().flush().unwrap();
    }
}
