use super::{plot_line, Direction, Entity};
use crate::debug;
use crate::map::Map;
use crate::textures::{PlayerTextures, Texture};
use crate::vectors::Vector;
use std::cell::RefCell;
use std::rc::Rc;
use termion::color;

/* Player */

// TODO remove pub
#[derive(Clone)]
pub struct Player {
    pub prev_point: Vector<f32>,
    pub point: Vector<f32>,
    pub velocity: Vector<f32>,
    pub name: String,
}

impl Player {
    pub fn new(x: f32, y: f32, name: &str) -> Player {
        Player {
            name: String::from(name),
            point: Vector { x: x, y: y },
            prev_point: Vector { x: x, y: y },
            velocity: Vector { x: 0.0, y: 0.0 },
        }
    }
}

impl Entity for Player {
    fn action(&mut self, direction: Direction) {
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
            _ => {
                return;
            }
        };

        self.velocity = self.velocity + to_add;
    }
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

    fn collide(&mut self, map: &Rc<RefCell<Map>>) {
        let (new_point, coll_opt) = plot_line(self.prev_point, self.point, Rc::clone(&map), true);

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

    fn get_color(&self) -> Option<&dyn color::Color> {
        Some(&color::Red)
    }

    fn get_point(&self) -> Vector<f32> {
        self.point
    }

    fn to_string(&self) -> &str {
        &self.name
    }

    fn update(&mut self) {
        let gravity_max = 3.0;
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
            if self.velocity.x != 0.0 {
                self.velocity.x += if self.velocity.x < 0.0 {
                    // if going left, add friction
                    friction_x
                } else {
                    // if going right, add friction
                    -1.0 * friction_x
                };
            }
        }

        // debug::write(&format!("v {}", self.velocity));

        self.prev_point = self.point;
        self.point = self.point + self.velocity;
    }
}
