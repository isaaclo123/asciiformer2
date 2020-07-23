use crate::consts::{EntityType, TEXTURE_MAP};
// use crate::linedraw::plot_line;
use crate::debug;
use crate::linedraw::plot_line;
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
    fn update(&mut self, stdout: &mut impl Write);
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

        // match self.get_texture() {
        //     PlayerTextures::NO_EXTEND => debug::write(stdout, "NO_EXTEND"),
        //     PlayerTextures::Y_EXTEND => debug::write(stdout, "Y_EXTEND"),
        //     PlayerTextures::X_EXTEND => debug::write(stdout, "X_EXTEND"),
        //     PlayerTextures::X_Y_EXTEND => debug::write(stdout, "X_Y_EXTEND"),
        //     _ => (),
        // };

        if let Some(c) = self.get_color() {
            write!(stdout, "{}", color::Fg(c)).unwrap();
        }

        let draw_pt = origin + self.get_point();
        let Vector {
            x: draw_x,
            y: draw_y,
        } = draw_pt;

        let texture = self.get_texture();

        // debug::write(
        //     stdout,
        //     &format!("texture_len x {} y {}", texture[0].len(), texture.len()),
        // );

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

    pub fn wall_collide(&mut self, stdout: &mut impl Write, map: &MapData) {
        let (new_point, coll_opt) = plot_line(stdout, self.prev_point, self.point, map);

        if let Some(coll_point) = coll_opt {
            let Vector { x: new_x, y: new_y } = new_point;
            let Vector {
                x: coll_x,
                y: coll_y,
            } = coll_point;

            if (coll_x - new_x).abs() <= 1.0 {
                // if collision occoured along x axis
                self.velocity.x = 0.0;
            }
            if (coll_y - new_y).abs() <= 1.0 {
                // if collision occoured along x axis
                self.velocity.y = 0.0;
            }
        }

        self.prev_point = new_point;
        self.point = new_point;
    }

    pub fn apply_gravity(&mut self, _stdout: &mut impl Write) {}

    pub fn action(&mut self, stdout: &mut impl Write, direction: Direction) {
        let speed = 2.0;
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

        self.velocity = self.velocity + to_add;
    }

    pub fn update(&mut self, stdout: &mut impl Write) {
        let gravity_max = 2.0;
        let gravity_inc = 0.25;
        let max_x_speed = 2.0;
        let friction_x = 0.1;

        // apply gravity
        if self.velocity.y < gravity_max {
            self.velocity.y += gravity_inc;
        }

        // apply left right speed
        if self.velocity.x.abs() > max_x_speed {
            self.velocity.x = if self.velocity.x < 0.0 {
                -1.0 * max_x_speed
            } else {
                max_x_speed
            };
        }

        // friction
        if self.velocity.x < gravity_max {
            self.velocity.x += if self.velocity.x < 0.0 {
                // if going left, add friction
                friction_x
            } else {
                // if going right, add friction
                -1.0 * friction_x
            };
        }

        debug::write(stdout, &format!("v {}", self.velocity));

        self.prev_point = self.point;
        self.point = self.point + self.velocity;
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

    fn update(&mut self, _stdout: &mut impl Write) {}
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
    fn update(&mut self, _stdout: &mut impl Write) {}
}
