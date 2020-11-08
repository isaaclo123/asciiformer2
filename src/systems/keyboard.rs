use crate::components::*;
use crate::resources::Map;
use crate::utils::Direction;
use euclid::default::Vector2D;
use specs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
use std::sync::{Arc, RwLock};

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        Entities<'a>,
        // Read<'a, Vector2D<u16>>,
        WriteStorage<'a, Position>,
        // Read<'a, Map>,
        WriteStorage<'a, Velocity>,
        ReadExpect<'a, Arc<RwLock<Vec<Direction>>>>,
        ReadStorage<'a, Speed>,
        WriteStorage<'a, MaxJump>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Color>,
        WriteStorage<'a, Texture>,
        WriteStorage<'a, Collide>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut position,
            mut velocity,
            movements,
            speed,
            mut max_jump,
            controlled,
            mut color,
            mut texture,
            mut collide,
        ): Self::SystemData,
    ) {
        let mut to_delete = 0;

        let mut bullets_to_add: Vec<(Position, Velocity, Color)> = vec![];

        for direction in movements.read().unwrap().iter() {
            for (pos, vel, spd, max_jmp, _, col) in (
                &position,
                &mut velocity,
                &speed,
                &mut max_jump,
                &controlled,
                &color,
            )
                .join()
            {
                if let Direction::To(mouse_pt) = direction {
                    // subtract 0.5, 0.5 to coutneracct (1,1) of terminal mouse point; center the
                    // aim of the point
                    let float_pt = mouse_pt.cast::<f32>() - Vector2D::new(0.5, 0.5);

                    bullets_to_add.push((
                        *pos,
                        Velocity::new_from_points(pos.0, float_pt, 2.0),
                        *col,
                    ));
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

        for (pos, vel, col) in bullets_to_add {
            let bullet = entities.create();
            position.insert(bullet, pos).unwrap();
            velocity.insert(bullet, vel).unwrap();
            color.insert(bullet, col).unwrap();
            texture
                .insert(bullet, Texture::new(&BulletTextures))
                .unwrap();
            collide
                .insert(bullet, Collide::new(OnCollideType::Delete))
                .unwrap();
        }

        for _ in 0..to_delete {
            movements.write().unwrap().remove(0);
        }
    }
}
