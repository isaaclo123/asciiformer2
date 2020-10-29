use crate::components::{Color, Position, Texture};
use crate::io::get_stdout;
use crate::resources::{Map, AIR, WALL};
use euclid::default::Vector2D;
use specs::{Join, Read, ReadStorage, System};
use std::io::Write;

use termion::{clear, color, cursor};

pub struct Renderer {
    draw_map: bool,
    to_clear: Vec<Vector2D<u16>>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            draw_map: true,
            to_clear: vec![],
        }
    }

    fn draw_map(&self, origin: Vector2D<u16>, map: &Map) -> () {
        for y in 0..map.height {
            for x in 0..map.width {
                if map.wall_get_unchecked(x as i16, y as i16) {
                    let draw_pt = origin + Vector2D::new(x as u16 + 1, y as u16 + 1);
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

    fn clear_prev(&mut self, origin: Vector2D<u16>, map: &Map) -> () {
        let draw_pt = origin + Vector2D::new(1, 1);

        for pt in self.to_clear.iter() {
            let clear_pt = draw_pt + *pt;

            let sym = if map.wall_get(pt.x as i16, pt.y as i16) {
                WALL
            } else {
                AIR
            };

            write!(
                get_stdout(),
                "{goto}{color}{sym}",
                goto = cursor::Goto(clear_pt.x, clear_pt.y),
                color = color::Fg(color::Reset),
                sym = sym
            )
            .unwrap();
        }

        self.to_clear.clear();
    }
}

impl<'a> System<'a> for Renderer {
    type SystemData = (
        Read<'a, Vector2D<u16>>,
        Read<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Color>,
        ReadStorage<'a, Texture>,
    );

    fn run(&mut self, (origin, map, position, color, texture): Self::SystemData) {
        let origin = *origin;

        if self.draw_map {
            write!(get_stdout(), "{}{}", clear::All, cursor::Hide).unwrap();
            self.draw_map(origin, &map);
            self.draw_map = false;
        }

        self.clear_prev(origin, &map);

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

                    let draw_pt = origin + Vector2D::new(1, 1) + texture_pt;

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
