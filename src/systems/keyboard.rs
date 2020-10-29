use crate::components::{KeyboardControlled, MaxJump, Speed, Velocity};
use crate::resources::Map;
use crate::utils::Direction;
use euclid::default::Vector2D;
use specs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
use std::sync::{Arc, RwLock};

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        Read<'a, Vector2D<u16>>,
        // Read<'a, Map>,
        WriteStorage<'a, Velocity>,
        ReadExpect<'a, Arc<RwLock<Vec<Direction>>>>,
        ReadStorage<'a, Speed>,
        WriteStorage<'a, MaxJump>,
        ReadStorage<'a, KeyboardControlled>,
    );

    fn run(
        &mut self,
        (origin, mut velocity, movements, speed, mut max_jump, controlled): Self::SystemData,
    ) {
        let mut to_delete = 0;

        for direction in movements.read().unwrap().iter() {
            for (vel, spd, max_jmp, _) in (&mut velocity, &speed, &mut max_jump, &controlled).join()
            {
                if let Direction::To(mouse_pt) = direction {
                    let int_pt = mouse_pt.cast::<i16>() - origin.cast::<i16>();
                    // subtract 0.5, 0.5 to coutneracct (1,1) of terminal mouse point; center the
                    // aim of the point
                    let float_pt = int_pt.cast::<f32>() - Vector2D::new(0.5, 0.5);
                } else {
                    let x_speed = spd.x_speed;
                    let y_speed = spd.y_speed;

                    let delta = match direction {
                        Direction::Up => {
                            if max_jmp.jump < max_jmp.max_jump {
                                max_jmp.jump += 1;
                                Vector2D::new(0.0, -1.0 * y_speed)
                            } else {
                                Vector2D::new(0.0, 0.0)
                            }
                        }
                        // Direction::Down => Vector2D::new(0.0, y_speed),
                        Direction::Left => Vector2D::new(-1.0 * x_speed, 0.0),
                        Direction::Right => Vector2D::new(x_speed, 0.0),
                        _ => Vector2D::new(0.0, 0.0),
                    };

                    vel.0 = vel.0 + delta;
                }
            }
            to_delete += 1;
        }

        for _ in 0..to_delete {
            movements.write().unwrap().remove(0);
        }
    }
}
