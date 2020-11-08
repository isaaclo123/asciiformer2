use crate::components::*;
use crate::resources::Map;
use crate::systems::collision::map_collision;
use euclid::default::Vector2D;
use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

pub struct Physics;

fn is_on_floor(map: &Map, pos: Vector2D<f32>) -> bool {
    let bottom = pos.y.floor() as i16 + 1;
    return map.wall_get(pos.x.ceil() as i16, bottom) || map.wall_get(pos.x.floor() as i16, bottom);
}

impl<'a> System<'a> for Physics {
    type SystemData = (
        Entities<'a>,
        Read<'a, Map>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, MaxSpeed>,
        WriteStorage<'a, MaxJump>,
        ReadStorage<'a, Gravity>,
        ReadStorage<'a, Friction>,
        ReadStorage<'a, Collide>,
    );

    fn run(
        &mut self,
        (
            entities,
            map,
            mut position,
            mut velocity,
            max_speed,
            mut max_jump,
            gravity,
            friction,
            collide,
        ): Self::SystemData,
    ) {
        for (ent, pos, vel, max_spd, max_jmp, grv, frc, coll) in (
            &entities,
            &mut position,
            &mut velocity,
            (&max_speed).maybe(),
            (&mut max_jump).maybe(),
            (&gravity).maybe(),
            (&friction).maybe(),
            (&collide).maybe(),
        )
            .join()
        {
            // let prev_pos = Vector2D::new(pos.0);
            let mut on_floor: Option<bool> = None;

            if let Some(j) = max_jmp {
                let cur_on_floor = is_on_floor(&map, pos.0);

                if cur_on_floor {
                    j.jump = 0;
                }

                on_floor = Some(cur_on_floor);
            }

            let prev_pos = pos.0;

            // apply friction
            if let Some(f) = frc {
                let cur_on_floor = if on_floor.is_none() {
                    let val = is_on_floor(&map, pos.0);
                    // on_floor = Some(val);
                    val
                } else {
                    on_floor.unwrap()
                };

                // only apply friction if character is on a floor
                if cur_on_floor {
                    let friction = f.friction;

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
                let gravity = g.gravity;

                vel.0.y += gravity;
            }

            // apply max_speeds
            if let Some(s) = max_spd {
                let max_x_speed = s.max_x_speed;
                let max_y_speed = s.max_y_speed;

                if vel.0.x.abs() > max_x_speed {
                    vel.0.x = s.max_x_speed * if vel.0.x < 0.0 { -1.0 } else { 1.0 };
                }
                if vel.0.y.abs() > max_y_speed {
                    vel.0.y = s.max_y_speed * if vel.0.y < 0.0 { -1.0 } else { 1.0 };
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

            if let Some(collide) = coll {
                match collide.on_collide {
                    OnCollideType::Delete => {
                        if coll_opt.is_some() {
                            entities.delete(ent);
                        } else {
                            pos.0 = new_pos;
                        }
                    }
                    OnCollideType::Block => {
                        pos.0 = new_point;
                    }
                }
            }
        }
    }
}
