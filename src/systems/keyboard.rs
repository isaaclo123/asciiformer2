use crate::components::{KeyboardControlled, Velocity};
use crate::utils::Direction;
use specs::prelude::*;
use specs::{Join, ReadExpect, ReadStorage, Storage, System, WriteStorage};
use vector2math::*;

pub struct Keyboard;

const PLAYER_SPEED: f32 = 2.0;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Vec<Direction>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (movements, controlled, velocity): Self::SystemData) {
        for (_, vel) in (&controlled, &mut velocity).join() {
            for direction in movements.iter() {
                let to_add = match direction {
                    Direction::Up => (0.0, -1.0 * PLAYER_SPEED),
                    Direction::Down => (0.0, PLAYER_SPEED),
                    Direction::Left => (-1.0 * PLAYER_SPEED, 0.0),
                    Direction::Right => (PLAYER_SPEED, 0.0),
                };

                (velocity).add((0, 1));
            }
        }
    }
}
