use super::{plot_line, Entity};
use crate::debug;
use crate::map::Map;
use crate::textures::{BulletTextures, Texture};
use crate::vectors::Vector;
use std::cell::RefCell;
use std::rc::Rc;
use termion::color;

/* Player */

// TODO remove pub
#[derive(Clone, Copy)]
pub struct Bullet {
    pub prev_point: Vector<f32>,
    pub point: Vector<f32>,
    pub velocity: Vector<f32>,
    should_remove: bool,
    // pub name: &'a str,
}

impl Bullet {
    // todo vectors maybe?
    pub fn new(p0: Vector<f32>, p1: Vector<f32>) -> Self {
        let speed = 3.5;
        // let velocity = ((p1 - p0) / p1.magn(p0)) * speed;
        // let velocity = (p1 - p0) / p0.magn(p1);
        let velocity = (p1 - p0) * speed / p0.magn(p1);

        debug::write(&format!("velocity {} ({} -> {})", velocity, p0, p1));

        Self {
            point: p0,
            prev_point: p0,
            velocity,
            should_remove: false,
        }
    }
}

impl Entity for Bullet {
    fn get_texture(&self) -> Texture {
        let Vector { x, y } = self.point;

        if x > 0.4 && x < 0.6 && y > 0.4 && y < 0.6 {
            return BulletTextures::MID;
        }

        match (y < 0.5, x < 0.5) {
            // is_top, is_left
            (true, true) => BulletTextures::TOP_LEFT,
            (false, true) => BulletTextures::BOT_LEFT,
            (true, false) => BulletTextures::TOP_RIGHT,
            (false, false) => BulletTextures::BOT_RIGHT,
        }
    }

    fn should_remove(&self) -> bool {
        self.should_remove
    }

    fn get_color(&self) -> Option<&dyn color::Color> {
        Some(&color::Blue)
    }

    fn get_point(&self) -> Vector<f32> {
        self.point
    }

    fn collide(&mut self, map: &Rc<RefCell<Map>>) {
        let (new_point, coll_opt) =
            plot_line(self.prev_point, self.point, Rc::clone(&map), false, true);

        if !self.should_remove {
            debug::write(&format!("newpoint {}", new_point));
        }

        if let Some(coll_point) = coll_opt {
            debug::write(&format!("collided on {}", coll_point));
            // TODO may be able to get rid of this
            self.prev_point = coll_point;
            self.point = coll_point;
            self.velocity = Vector::new(0.0, 0.0);
            self.should_remove = true;
            return;
        }

        // self.prev_point = new_point;
        // self.point = new_point;
    }

    fn to_string(&self) -> &str {
        "Bullet"
    }

    fn update(&mut self) {
        self.prev_point = self.point;
        self.point = self.point + self.velocity;
    }
}
