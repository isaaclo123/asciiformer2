use crate::components::{Friction, Gravity, Position, Speed, Velocity};
use euclid::default::Vector2D;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Speed>,
        ReadStorage<'a, Gravity>,
        ReadStorage<'a, Friction>,
    );

    fn run(&mut self, (mut position, mut velocity, speed, gravity, friction): Self::SystemData) {
        for (pos, vel, spd, grv, frc) in (
            &mut position,
            &mut velocity,
            (&speed).maybe(),
            (&gravity).maybe(),
            (&friction).maybe(),
        )
            .join()
        {
            let prev_pos = pos.0;

            // apply friction
            if let Some(f) = frc {
                let friction = f.get_friction();
                if vel.0.x != 0.0 {
                    vel.0.x += if vel.0.x < 0.0 {
                        // if going left, add friction
                        friction
                    } else {
                        // if going right, add friction
                        -1.0 * friction
                    };
                }
            }

            // apply max_speed
            if let Some(s) = spd {
                let max_speed = s.get_max_speed();

                if vel.0.x.abs() > max_speed {
                    vel.0.x = if vel.0.x < 0.0 {
                        -1.0 * max_speed
                    } else {
                        max_speed
                    };
                }
            }

            // apply gravity
            if let Some(g) = grv {
                let gravity = g.get_gravity();
                let max_gravity = g.get_max_gravity();

                if vel.0.y < max_gravity {
                    vel.0.y += gravity;
                }
                if vel.0.y > max_gravity {
                    vel.0.y += max_gravity;
                }
            }

            pos.0 = pos.0 + vel.0
        }

        /*
        for (_, vel) in (&mut velocity).join() {
            // apply gravity
            if vel.0.y < GRAVITY_MAX {
                vel.0.y += GRAVITY;
            }

            // apply left right speed
            if vel.0.x.abs() > MAX_PLAYER_SPEED {
                vel.0.x = if vel.0.x < 0.0 {
                    -1.0 * MAX_PLAYER_SPEED
                } else {
                    MAX_PLAYER_SPEED
                };
            }

            // friction
            if vel.0.x != 0.0 {
                vel.0.x += if vel.0.x < 0.0 {
                    // if going left, add friction
                    FRICTION
                } else {
                    // if going right, add friction
                    -1.0 * FRICTION
                };
            }
        }
        */
    }
}
