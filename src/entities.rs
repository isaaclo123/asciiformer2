use crate::consts::{EntityType, TEXTURE_MAP};
use crate::map::MapData;
use crate::textures::{AirTextures, PlayerTextures, Texture, WallTextures};
use crate::vectors::Vector;
use std::io::Write;
use termion::{clear, color, cursor};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait Entity<'a> {
    // fn update(&mut self);
    // fn get_x(&mut self) -> u16;
    // fn get_y(&mut self) -> u16;
    // fn draw(&mut self, stdout: &'a mut impl Write);
    fn collide(&mut self, entity: &'a mut impl Entity<'a>);

    // fn should_draw(&self) -> bool;

    fn to_string(&self) -> &'a str;
    fn get_texture(&self) -> Texture;
    fn get_color(&self) -> Option<&'a dyn color::Color>;
    fn get_point(&self) -> Vector<u16>;

    fn clear(&self, stdout: &mut impl Write, origin: Vector<u16>, map: &MapData) {
        // if !self.should_draw() {
        //     return;
        // }

        let Vector {
            x: point_x,
            y: point_y,
        } = self.get_point();
        let draw_pt = origin + self.get_point();
        let Vector {
            x: draw_x,
            y: draw_y,
        } = draw_pt;

        let texture = self.get_texture();

        for texture_y in 0..texture.len() {
            for texture_x in 0..texture[texture_y].len() {
                let bg_texture = map.get(&(texture_x as u16 + point_x, texture_y as u16 + point_y));
                // AirTextures::AIR_CHAR

                let sym = if let Some(e) = bg_texture {
                    e.get_texture()[0][0]
                } else {
                    AirTextures::AIR[0][0]
                };

                writeln!(
                    stdout,
                    "{goto}{sym}",
                    goto = cursor::Goto(
                        draw_x + 1 + texture_x as u16,
                        draw_y as u16 + 1 + texture_y as u16
                    ),
                    sym = sym
                )
                .unwrap();
            }
        }

        // TODO overflow issue

        write!(stdout, "{}", color::Fg(color::Reset)).unwrap();
    }

    fn draw(
        &self,
        stdout: &mut impl Write,
        origin: Vector<u16>,
        // point: Vector<i16>,
        // fg_opt: Option<impl color::Color>,
    ) {
        // if !self.should_draw() {
        //     return;
        // }

        match self.get_texture() {
            PlayerTextures::NO_EXTEND => {
                writeln!(stdout, "{}NO_EXTEND", cursor::Goto(1, 1)).unwrap()
            }
            PlayerTextures::Y_EXTEND => writeln!(stdout, "{}Y_EXTEND", cursor::Goto(1, 1)).unwrap(),
            PlayerTextures::X_EXTEND => writeln!(stdout, "{}X_EXTEND", cursor::Goto(1, 1)).unwrap(),
            PlayerTextures::X_Y_EXTEND => {
                writeln!(stdout, "{}X_Y_EXTEND", cursor::Goto(1, 1)).unwrap()
            }
            _ => (),
        };

        if let Some(c) = self.get_color() {
            write!(stdout, "{}", color::Fg(c)).unwrap();
        }

        let draw_pt = origin + self.get_point();
        let Vector {
            x: draw_x,
            y: draw_y,
        } = draw_pt;

        let texture = self.get_texture();

        writeln!(
            stdout,
            "{}xlen {} ylen {}",
            cursor::Goto(1, 2),
            texture[0].len(),
            texture.len()
        )
        .unwrap();

        for y in 0..texture.len() {
            for x in 0..texture[y].len() {
                writeln!(
                    stdout,
                    "{goto}{sym}",
                    goto = cursor::Goto(draw_x + 1 + x as u16, draw_y as u16 + 1 + y as u16),
                    sym = texture[y][x]
                )
                .unwrap();
            }
        }

        // TODO overflow issue

        write!(stdout, "{}", color::Fg(color::Reset)).unwrap();
    }
}

// pub fn bresenham_line(p0: (f32, f32), p1: (f32, f32)) -> Vec<(u16, u16)> {
//     let (x0, y0) = p0;
//     let (x1, y1) = p1;
//
//     let dx = x1 - x0;
//     let dy = y1 - y0;
//
//     let mut points = Vec::new();
//
//     if dx.abs() > dy.abs() {
//         points.push()
//     }
// }

/* Player */

// TODO remove pub
pub struct Player<'a> {
    pub prev_point: Vector<f32>,
    pub point: Vector<f32>,
    pub velocity: Vector<f32>,
    pub name: &'a str,
}

impl<'a> Player<'a> {
    pub fn new(x: f32, y: f32, name: &'a str) -> Player<'a> {
        Player {
            name: name,
            point: Vector { x: x, y: y },
            prev_point: Vector { x: x, y: y },
            velocity: Vector { x: 0.0, y: 0.0 },
        }
    }

    // TODO
    pub fn wall_collide(&mut self, stdout: &mut impl Write, map: &MapData) {
        // if self.point == self.prev_point {
        //     return;
        // }

        let Vector {
            x: diff_x,
            y: diff_y,
        } = self.point - self.prev_point;

        let mut ratio: f32;
        let mut inc_vec: Vector<f32>;

        let ratio = diff_y / diff_x; // ratio of y / x

        // if diff_x == 0.0 {}

        // vector to increment check by
        let inc_vec = if self.prev_point.x < self.point.x {
            Vector { x: 1.0, y: ratio }
        } else {
            Vector {
                x: -1.0,
                y: -1.0 * ratio,
            }
        };

        let mut prev_new_point = self.prev_point;
        let mut new_point = self.prev_point;

        let mut index = 0;

        loop {
            if self.prev_point.x < self.point.x && new_point.x >= self.point.x {
                self.prev_point = self.point;
                break;
            }
            if self.prev_point.x > self.point.x && new_point.x <= self.point.x {
                self.prev_point = self.point;
                break;
            }

            let x_index = new_point.x as u16;

            let y_ceil = new_point.y.ceil() as u16;
            let y_floor = new_point.y.floor() as u16;

            writeln!(
                stdout,
                "{}NEW_X {:.3} NEW_Y {:.3} RATIO {}|",
                cursor::Goto(1, 3),
                new_point.x,
                new_point.y,
                ratio
            )
            .unwrap();
            writeln!(
                stdout,
                "{}PREV_X {:.3} PREV_Y {:.3}|",
                cursor::Goto(1, 4),
                self.prev_point.x,
                self.prev_point.y,
            )
            .unwrap();
            writeln!(
                stdout,
                "{}POINT_X {:.3} POINT_Y {:.3}|",
                cursor::Goto(1, 5),
                self.point.x,
                self.point.y,
            )
            .unwrap();
            writeln!(stdout, "{}{}|", cursor::Goto(1, 6), index,).unwrap();

            index += 1;

            let result_floor = map.get(&(x_index, y_floor));
            let result_ceil = map.get(&(x_index, y_ceil));

            if result_floor.is_some() || result_ceil.is_some() {
                self.point = prev_new_point;
                self.point.x = self.point.x.round();
                self.prev_point = self.point;
                break;
            }

            prev_new_point = new_point;
            new_point += inc_vec;

            /*


            */

            if let Some(e) = result_ceil {
                writeln!(
                    stdout,
                    "{}C({},{},{})",
                    cursor::Goto(1 + x_index, 1 + y_ceil),
                    x_index,
                    y_ceil,
                    e.get_texture()[0][0]
                )
                .unwrap();
            } else {
                writeln!(
                    stdout,
                    "{}C({},{})",
                    cursor::Goto(1 + x_index, 1 + y_ceil),
                    x_index,
                    y_ceil
                )
                .unwrap();
            }

            if let Some(e) = result_floor {
                writeln!(
                    stdout,
                    "{}C({},{},{})",
                    cursor::Goto(1 + x_index, 1 + y_floor),
                    x_index,
                    y_floor,
                    e.get_texture()[0][0]
                )
                .unwrap();
            } else {
                writeln!(
                    stdout,
                    "{}C({},{})",
                    cursor::Goto(1 + x_index, 1 + y_floor),
                    x_index,
                    y_floor
                )
                .unwrap();
            }
        }

        // let iter_y = if self.prev_point.y < self.point.y {
        //     1
        // } else {
        //     -1
        // };
    }

    pub fn action(&mut self, direction: Direction) {
        let speed = 10.15;
        let to_add = match direction {
            Direction::Up => Vector {
                x: 0.0,
                y: -1.0 * speed,
            },
            Direction::Down => Vector { x: 0.0, y: speed },
            Direction::Left => Vector {
                x: -1.0 * speed,
                y: 0.0,
            },
            Direction::Right => Vector { x: speed, y: 0.0 },
        };

        self.prev_point = self.point;
        self.point = self.point + to_add;
    }
}

impl<'a> Entity<'a> for Player<'a> {
    // fn should_draw(&self) -> bool {
    //     !(self.prev_point.round_int() == self.point.round_int())
    // }
    fn get_texture(&self) -> Texture {
        let floor_pt = self.point.floor_int();
        let round_pt = self.point.round_int();

        let texture = match (floor_pt.x == round_pt.x, floor_pt.y == round_pt.y) {
            (true, true) => PlayerTextures::NO_EXTEND,
            (true, false) => PlayerTextures::Y_EXTEND,
            (false, true) => PlayerTextures::X_EXTEND,
            (false, false) => PlayerTextures::X_Y_EXTEND,
        };

        texture
    }

    fn get_color(&self) -> Option<&'a dyn color::Color> {
        Some(&color::Red)
    }

    fn get_point(&self) -> Vector<u16> {
        self.point.floor_int()
    }

    fn collide(&mut self, _entity: &'a mut impl Entity<'a>) {}

    fn to_string(&self) -> &'a str {
        self.name
    }
    // fn update(&mut self) {}
}

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
    // fn should_draw(&self) -> bool {
    //     true
    // }

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
    // fn update(&mut self) {}
}
