use crate::components::{KeyboardControlled, Speed, Velocity};
use crate::utils::Direction;
use euclid::default::Vector2D;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};
use std::sync::{Arc, RwLock};

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Arc<RwLock<Vec<Direction>>>>,
        ReadStorage<'a, Speed>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (movements, speed, controlled, mut velocity): Self::SystemData) {
        let mut to_delete = 0;

        for direction in movements.read().unwrap().iter() {
            for (spd, _, vel) in (&speed, &controlled, &mut velocity).join() {
                let speed = spd.get_speed();

                let delta = match direction {
                    Direction::Up => Vector2D::new(0.0, -1.0 * speed),
                    Direction::Down => Vector2D::new(0.0, speed),
                    Direction::Left => Vector2D::new(-1.0 * speed, 0.0),
                    Direction::Right => Vector2D::new(speed, 0.0),
                };
                // let delta = Vector2D::new(0.1, 0.0);

                vel.0 = vel.0 + delta;

                // if vel.0.x.abs() > max_speed {
                //     vel.0.x = if vel.0.x < 0.0 {
                //         -1.0 * max_speed
                //     } else {
                //         max_speed
                //     };
                // }
            }
            to_delete += 1;
        }

        for _ in 0..to_delete {
            movements.write().unwrap().remove(0);
        }
    }
}
