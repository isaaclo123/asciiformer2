use crate::consts::{EntityType, TEXTURE_MAP};
// use crate::linedraw::plot_line;
use crate::debug;
use crate::game::{Game, GameInfo};
use crate::linedraw::plot_line;
use crate::map::MapData;
use crate::textures::{AirTextures, PlayerTextures, Texture, WallTextures};
use crate::vectors::Vector;
use std::cell::RefCell;
use std::io::{stdin, stdout, Read, Write};
use std::rc::Rc;
use termion::{clear, color, cursor};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait Entity<'a, R: Read + 'a, W: Write + 'a> {
    fn update(&mut self);

    // fn collide(&mut self, entity: &'a mut impl Entity<'a, R, W>);

    fn to_string(&self) -> &'a str;
    fn get_texture(&self) -> Texture;
    fn get_game_info(&self) -> &RefCell<GameInfo<'a, R, W>>;
    fn get_color(&self) -> Option<&'a dyn color::Color>;
    fn get_point(&self) -> Vector<u16>;

    fn clear(&self) {
        // if !self.should_draw() {
        //     return;
        // }
        //
        let origin = &self.get_game_info().borrow().origin;
        let map = &self.get_game_info().borrow().map;

        let Vector {
            x: point_x,
            y: point_y,
        } = self.get_point();
        let draw_pt = *origin + self.get_point();
        let Vector {
            x: draw_x,
            y: draw_y,
        } = draw_pt;

        let texture = self.get_texture();

        for texture_y in 0..texture.len() {
            for texture_x in 0..texture[texture_y].len() {
                let bg_texture = map.get((texture_x as u16 + point_x, texture_y as u16 + point_y));

                // AirTextures::AIR_CHAR

                let sym = if let Some(e) = bg_texture {
                    e.get_texture()[0][0]
                } else {
                    AirTextures::AIR[0][0]
                };

                writeln!(
                    self.get_game_info().borrow_mut().stdout,
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

        write!(
            self.get_game_info().borrow_mut().stdout,
            "{}",
            color::Fg(color::Reset)
        )
        .unwrap();
    }

    fn draw(
        &self,
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
        //
        let origin = &self.get_game_info().borrow().origin;

        if let Some(c) = self.get_color() {
            write!(self.get_game_info().borrow_mut().stdout, "{}", color::Fg(c)).unwrap();
        }

        let draw_pt = *origin + self.get_point();
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
                    self.get_game_info().borrow_mut().stdout,
                    "{goto}{sym}",
                    goto = cursor::Goto(draw_x + 1 + x as u16, draw_y as u16 + 1 + y as u16),
                    sym = texture[y][x]
                )
                .unwrap();
            }
        }

        // TODO overflow issue

        write!(
            self.get_game_info().borrow_mut().stdout,
            "{}",
            color::Fg(color::Reset)
        )
        .unwrap();
    }
}

/* Player */

// TODO remove pub
pub struct Player<'a, R, W> {
    pub prev_point: Vector<f32>,
    pub point: Vector<f32>,
    pub velocity: Vector<f32>,
    pub name: &'a str,
    pub game_info: Rc<RefCell<GameInfo<'a, R, W>>>,
}

impl<'a, R: Read, W: Write> Player<'a, R, W> {
    pub fn new(
        game_info: Rc<RefCell<GameInfo<'a, R, W>>>,
        point: Vector<f32>,
        name: &'a str,
    ) -> Self {
        Player {
            // game: game,
            game_info,
            name,
            point,
            prev_point: point,
            velocity: Vector { x: 0.0, y: 0.0 },
        }
    }

    pub fn wall_collide(&mut self) {
        let (new_point, coll_opt) = plot_line(self.get_game_info(), self.prev_point, self.point);

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

    pub fn action(&mut self, _stdout: &mut impl Write, direction: Direction) {
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

    pub fn update(&mut self) {
        let gravity_max = 2.0;
        let gravity_inc = 0.25;
        let max_x_speed = 2.0;
        let friction_x = 0.25;

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
            } else if self.velocity.x > 0.0 {
                // if going right, reduce friction
                -1.0 * friction_x
            } else {
                0.0
            };
        }

        debug::write(
            self.get_game_info().borrow_mut().stdout,
            &format!("v {}", self.velocity),
        );

        self.prev_point = self.point;
        self.point = self.point + self.velocity;
    }
}

impl<'a, R: Read, W: Write> Entity<'a, R, W> for Player<'a, R, W> {
    // fn should_draw(&self) -> bool {
    //     !(self.prev_point.round_int() == self.point.round_int())
    // }

    fn get_game_info(&self) -> &RefCell<GameInfo<'a, R, W>> {
        &self.game_info
    }
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

    // fn collide(&mut self, _entity: &'a mut impl Entity<'a>) {}

    fn to_string(&self) -> &'a str {
        self.name
    }

    fn update(&mut self) {}
}

/* Walls */

pub struct Wall<'a, R, W> {
    pub point: Vector<f32>,
    pub game_info: Rc<RefCell<GameInfo<'a, R, W>>>,
}

impl<'a, R: Read, W: Write> Wall<'a, R, W> {
    pub fn new(game_info: Rc<RefCell<GameInfo<'a, R, W>>>, point: Vector<f32>) -> Self {
        Wall {
            // game: game,
            game_info,
            point,
        }
    }
}

impl<'a, R: Read, W: Write> Entity<'a, R, W> for Wall<'a, R, W> {
    // fn should_draw(&self) -> bool {
    //     true
    // }
    fn get_game_info(&self) -> &RefCell<GameInfo<'a, R, W>> {
        &self.game_info
    }

    fn get_texture(&self) -> Texture {
        return WallTextures::WALL;
    }

    fn get_color(&self) -> Option<&'a dyn color::Color> {
        None
    }

    fn get_point(&self) -> Vector<u16> {
        self.point.floor_int()
    }

    // fn collide(&mut self, _entity: &'a mut impl Entity<'a>) {}

    fn to_string(&self) -> &'a str {
        "Wall"
    }
    fn update(&mut self) {}
}
