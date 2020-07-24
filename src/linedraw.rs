use crate::debug;
use crate::game::GameInfo;
use crate::map::{Map, MapData};
use crate::vectors::Vector;
use std::cell::RefCell;
use std::io::{Read, Write};

fn plot_line_low<'a, R: Read, W: Write>(
    game_info: &RefCell<GameInfo<'a, R, W>>,
    p0: Vector<f32>,
    p1: Vector<f32>,
) -> (Vector<f32>, Option<Vector<f32>>) {
    let Vector { x: x0, y: y0 } = p0;
    let Vector { x: x1, y: y1 } = p1;

    let stdout = game_info.borrow_mut().stdout;

    // debug::write(stdout, &format!("first {}", p0));

    let (dx, dy) = (x1 - x0, y1 - y0);
    // let Vector { x: dx, dy: dy } = p1 - p0;

    let slope = dy / dx;

    let xi: i16;
    let di: f32;
    let x0_f: f32;
    let x1_f: f32;
    let x0_i: i16;
    let x1_i: i16;

    if dx < 0.0 {
        debug::write(stdout, &format!("Left {}<-{}", p1, p0));
        // going left
        x0_f = x0.ceil();
        x1_f = x1.floor();
        xi = -1;
        di = -1.0 * slope;
    } else {
        debug::write(stdout, &format!("Right {}->{}", p0, p1));
        // going right
        x0_f = x0.floor();
        x1_f = x1.ceil();
        xi = 1;
        di = slope;
    };
    x0_i = x0_f as i16;
    x1_i = x1_f as i16;
    // //
    // debug::write(stdout, "PLOT_LOW-------------");
    // debug::write(stdout, &format!("dy {} dx {}", dy, dx));
    // debug::write(stdout, &format!("slope {}", slope));
    // debug::write(stdout, &format!("di {}", di));
    // debug::write(stdout, &format!("y0 {}", y0));
    // debug::write(stdout, "******");
    // debug::write(stdout, &format!("x0 {} x1 {}", x0, x1));
    // debug::write(stdout, &format!("x0_f {}, x1_f {}", x0_f, x1_f));
    // debug::write(stdout, &format!("x0_i {}, x1_i {}", x0_i, x1_i));
    // debug::write(stdout, "PLOT_LOW-------------");

    // decimal place of x
    let dec_x = x0 - x0_f;

    // let mut y = y0 + di * dec_x;
    let mut y = y0 + di * dec_x;

    let mut prev_vec = p0;

    let mut x = x0_i;
    let mut end = false;

    // let mut straight_check = true;

    loop {
        if x == x1_i {
            // if incrementing and x greater than x1, break
            // also, should not be straight_check, should be checking with slope change
            end = true
        }

        let y_floor = y.floor();
        let y_ceil = y.ceil();
        let y_round = prev_vec.y.round();

        let check_order = if dy > 0.0
        // going upwards
        {
            [y_round, y_floor, y_ceil]
        } else {
            [y_round, y_ceil, y_floor]
        };

        let mut collide_pos = None;
        let mut modified = false;

        for y_check in check_order.iter() {
            let check = game_info.borrow().map.get((x as u16, *y_check as u16));

            if check.is_some() {
                debug::write(stdout, &format!("check ({}, {}) W", x, y_check));
                // if area is unable to be walked into
                collide_pos = Some(Vector {
                    x: x as f32,
                    y: *y_check,
                });
                break;
            }
            debug::write(stdout, &format!("check ({}, {})", x, y_check));
            modified = true;
            y = *y_check;
        }
        debug::write(stdout, "----");

        if !modified && collide_pos.is_some() {
            return (prev_vec, collide_pos);
        }

        prev_vec = Vector { x: x as f32, y: y };

        if end {
            break;
        }

        x += xi;
        if collide_pos.is_none() {
            y += dy;
        }
    }

    return (prev_vec, None);
}

fn plot_line_high<'a, R: Read, W: Write>(
    game_info: &RefCell<GameInfo<'a, R, W>>,
    p0: Vector<f32>,
    p1: Vector<f32>,
) -> (Vector<f32>, Option<Vector<f32>>) {
    let Vector { x: x0, y: y0 } = p0;
    let Vector { x: x1, y: y1 } = p1;

    let stdout = &game_info.borrow_mut().stdout;

    debug::write(*stdout, "PLOT LINE HIGH");

    let (dx, dy) = (x1 - x0, y1 - y0);

    let slope = dx / dy;

    let yi: i16;
    let di: f32;
    let y0_f: f32;
    let y1_f: f32;
    let y0_i: i16;
    let y1_i: i16;

    if dy < 0.0 {
        debug::write(*stdout, &format!("Up {}^{}", p0, p1));
        // going up
        y0_f = y0.ceil();
        y1_f = y1.floor();
        yi = -1;
        di = -1.0 * slope;
    } else {
        debug::write(*stdout, &format!("Down {}V{}", p0, p1));
        // going down
        y0_f = y0.floor();
        y1_f = y1.ceil();
        yi = 1;
        di = slope;
    };
    y0_i = y0_f as i16;
    y1_i = y1_f as i16;

    // debug::write(stdout, "PLOT_HIGH-----------");
    // debug::write(stdout, &format!("dy {} dx {}", dy, dx));
    // debug::write(stdout, &format!("slope dx/dy {}", slope));
    // debug::write(stdout, &format!("di {}", di));
    // debug::write(stdout, &format!("x0 {}", y0));
    // debug::write(stdout, "******");
    // debug::write(stdout, &format!("y0 {} y1 {}", y0, y1));
    // debug::write(stdout, &format!("y0_f {}, y1_f {}", y0_f, y1_f));
    // debug::write(stdout, &format!("y0_i {}, y1_i {}", y0_i, y1_i));
    // debug::write(stdout, "PLOT_HIGH------------");

    // decimal place of x
    let dec_y = y0 - y0_f;

    let mut x = x0 + di * dec_y;

    let mut prev_vec = p0;

    let mut y = y0_i;
    let mut end = false;

    loop {
        if y == y1_i {
            // if incrementing and x greater than x1, break
            // also, should not be straight_check, should be checking with slope change
            end = true
        }

        let x_floor = x.floor();
        let x_ceil = x.ceil();
        let x_round = prev_vec.x.round();

        let check_order = if dx > 0.0
        // going upwards
        {
            [x_round, x_floor, x_ceil]
        } else {
            [x_round, x_ceil, x_floor]
        };

        let mut collide_pos = None;
        let mut modified = false;

        for x_check in check_order.iter() {
            let map = &game_info.borrow().map;
            let check = map.get((*x_check as u16, y as u16));

            if check.is_some() {
                debug::write(*stdout, &format!("check ({}, {}) W", x_check, y));
                // if area is unable to be walked into
                collide_pos = Some(Vector {
                    x: *x_check,
                    y: y as f32,
                });
                break;
            }
            debug::write(*stdout, &format!("check ({}, {})", x_check, y));
            modified = true;
            x = *x_check;
        }
        debug::write(*stdout, "----");

        if !modified && collide_pos.is_some() {
            return (prev_vec, collide_pos);
        }

        prev_vec = Vector { x: x, y: y as f32 };

        if end {
            break;
        }

        y += yi;
        if collide_pos.is_none() {
            x += dx;
        }
    }

    return (prev_vec, None);
}

pub fn plot_line<'a, R: Read, W: Write>(
    game_info: &RefCell<GameInfo<'a, R, W>>,
    p0: Vector<f32>,
    p1: Vector<f32>,
) -> (Vector<f32>, Option<Vector<f32>>) {
    let Vector { x: x0, y: y0 } = p0;
    let Vector { x: x1, y: y1 } = p1;

    if p0 == p1 {
        return (p0, None);
    }

    if (y1 - y0).abs() < (x1 - x0).abs() {
        return plot_line_low(game_info, p0, p1);
    } else {
        return plot_line_high(game_info, p0, p1);
    }
}
