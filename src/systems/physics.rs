use crate::components::{Friction, Gravity, MaxSpeed, Position, Velocity};
use crate::resources::Map;
use crate::systems::collision::map_collision;
use euclid::default::Vector2D;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (
        Read<'a, Map>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, MaxSpeed>,
        ReadStorage<'a, Gravity>,
        ReadStorage<'a, Friction>,
    );

    fn run(
        &mut self,
        (map, mut position, mut velocity, max_speed, gravity, friction): Self::SystemData,
    ) {
        for (pos, vel, max_spd, grv, frc) in (
            &mut position,
            &mut velocity,
            (&max_speed).maybe(),
            (&gravity).maybe(),
            (&friction).maybe(),
        )
            .join()
        {
            // let prev_pos = Vector2D::new(pos.0);

            let prev_pos = pos.0;

            // apply friction
            if let Some(f) = frc {
                // only apply friction if character is on a floor
                if map.wall_get(pos.0.x.ceil() as i16, pos.0.y.floor() as i16 + 1)
                    || map.wall_get(pos.0.x.floor() as i16, pos.0.y.floor() as i16 + 1)
                {
                    let friction = f.get_friction();

                    if vel.0.x.abs() <= friction {
                        vel.0.x = 0.0;
                    } else {
                        vel.0.x += friction
                            * if vel.0.x < 0.0 {
                                // if going left, add friction
                                1.0
                            } else {
                                // if going right, add friction
                                -1.0
                            };
                    }
                }
            }

            // apply gravity
            if let Some(g) = grv {
                let gravity = g.get_gravity();

                vel.0.y += gravity;
            }

            // apply max_speeds
            if let Some(s) = max_spd {
                let max_x_speed = s.get_max_x_speed();
                let max_y_speed = s.get_max_y_speed();

                if vel.0.x.abs() > max_x_speed {
                    vel.0.x = max_x_speed * if vel.0.x < 0.0 { -1.0 } else { 1.0 };
                }
                if vel.0.y.abs() > max_y_speed {
                    vel.0.y = max_y_speed * if vel.0.y < 0.0 { -1.0 } else { 1.0 };
                }
            }

            let new_pos = pos.0 + vel.0;

            let (new_point, coll_opt) = map_collision(prev_pos, new_pos, &map, true, true);

            if let Some(coll_point) = coll_opt {
                let Vector2D {
                    x: new_x, y: new_y, ..
                } = new_point;
                let Vector2D {
                    x: coll_x,
                    y: coll_y,
                    ..
                } = coll_point;

                if (coll_x - new_x).abs() <= 1.0 {
                    // if collision occoured along x axis
                    vel.0.x = 0.0;
                    // if self.velocity.x < 0.1 {
                    //     self.velocity.x = 0.0;
                    // }
                    // self.velocity.x *= -0.25;
                }
                if (coll_y - new_y).abs() <= 1.0 {
                    vel.0.y = 0.0;
                    // if self.velocity.y < 0.1 {
                    //     self.velocity.y = 0.0;
                    // }
                    // // if collision occoured along x axis
                    // if self.velocity.y > 1.0 {
                    //     self.velocity.y *= -0.5;
                    // }
                }
            }

            pos.0 = new_point;
        }
    }
}
