use crate::components::{KeyboardControlled, Velocity};
use crate::utils::Direction;
use euclid::default::Vector2D;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};
use std::sync::{Arc, RwLock};

pub struct Keyboard;

const PLAYER_SPEED: f32 = 2.0;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Arc<RwLock<Vec<Direction>>>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (movements, controlled, mut velocity): Self::SystemData) {
        for (_, vel) in (&controlled, &mut velocity).join() {
            for direction in movements.read().unwrap().iter() {
                let delta = match direction {
                    Direction::Up => Vector2D::new(0.0, -1.0 * PLAYER_SPEED),
                    Direction::Down => Vector2D::new(0.0, PLAYER_SPEED),
                    Direction::Left => Vector2D::new(-1.0 * PLAYER_SPEED, 0.0),
                    Direction::Right => Vector2D::new(PLAYER_SPEED, 0.0),
                };

                vel.0 = vel.0 + delta;
            }
        }
    }
}
