use crate::debug;
use crate::map::MapData;
use crate::vectors::Vector;
use std::io::Write;

// fn plot_line_low((x0, y0): (i16, i16), (x1, y1): (i16, i16)) -> Vec<(i16, i16)> {
fn plot_line_low(
    stdout: &mut impl Write,
    p0: Vector<f32>,
    p1: Vector<f32>,
    map: &MapData,
) -> Vector<f32> {
    let Vector { x: x0, y: y0 } = p0;
    let Vector { x: x1, y: y1 } = p1;

    // debug::write(stdout, &format!("first {}", p0));

    let (dx, dy) = (x1 - x0, y1 - y0);
    // let Vector { x: dx, dy: dy } = p1 - p0;
    //
    if dy == 0.0 && dx == 0.0 {
        return p0;
    }

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
    //
    debug::write(stdout, "PLOT_LOW-------------");
    debug::write(stdout, &format!("dy {} dx {}", dy, dx));
    debug::write(stdout, &format!("slope {}", slope));
    debug::write(stdout, &format!("di {}", di));
    debug::write(stdout, &format!("y0 {}", y0));
    debug::write(stdout, "******");
    debug::write(stdout, &format!("x0 {} x1 {}", x0, x1));
    debug::write(stdout, &format!("x0_f {}, x1_f {}", x0_f, x1_f));
    debug::write(stdout, &format!("x0_i {}, x1_i {}", x0_i, x1_i));
    debug::write(stdout, "PLOT_LOW-------------");

    // decimal place of x
    let dec_x = x0 - x0_f;

    let mut y = y0 + di * dec_x;

    let mut prev_vec = p0;

    let mut x = x0_i;
    let mut end = false;

    loop {
        if x == x1_i {
            // if incrementing and x greater than x1, break
            end = true
        }

        // debug::write(stdout, "In Loop");

        let y_ceil = y.ceil();
        let y_floor = y.floor();

        let map_ceil = map.get(&(x as u16, y_ceil as u16));
        let map_floor = map.get(&(x as u16, y_floor as u16));

        if map_ceil.is_some() {
            debug::write(stdout, &format!("({}, {}) 'W' ceil", x, y_ceil));
            return prev_vec;
        } else if map_floor.is_some() {
            debug::write(stdout, &format!("({}, {}) 'W' floor", x, y_floor));
            return prev_vec;
        } else {
            debug::write(stdout, &format!("({}, {}) ' ' ceil", x, y_ceil));
            debug::write(stdout, &format!("({}, {}) ' ' floor", x, y_floor));
        }

        if end {
            break;
        }

        prev_vec = Vector { x: x as f32, y: y };

        x += xi;
        y += dy;
    }

    return p1;
}

fn plot_line_high(
    stdout: &mut impl Write,
    p0: Vector<f32>,
    p1: Vector<f32>,
    map: &MapData,
) -> Vector<f32> {
    let Vector { x: x0, y: y0 } = p0;
    let Vector { x: x1, y: y1 } = p1;

    debug::write(stdout, "PLOT LINE HIGH");

    let (dx, dy) = (x1 - x0, y1 - y0);
    // let Vector { x: dx, dy: dy } = p1 - p0;
    //
    if dy == 0.0 && dx == 0.0 {
        return p0;
    }

    let slope = dx / dy;

    let yi: i16;
    let di: f32;
    let y0_f: f32;
    let y1_f: f32;
    let y0_i: i16;
    let y1_i: i16;

    if dy < 0.0 {
        debug::write(stdout, &format!("Up {}^{}", p0, p1));
        // going up
        y0_f = y0.ceil();
        y1_f = y1.floor();
        yi = -1;
        di = -1.0 * slope;
    } else {
        debug::write(stdout, &format!("Down {}V{}", p0, p1));
        // going down
        y0_f = y0.floor();
        y1_f = y1.ceil();
        yi = 1;
        di = slope;
    };
    y0_i = y0_f as i16;
    y1_i = y1_f as i16;

    debug::write(stdout, "PLOT_HIGH-----------");
    debug::write(stdout, &format!("dy {} dx {}", dy, dx));
    debug::write(stdout, &format!("slope dx/dy {}", slope));
    debug::write(stdout, &format!("di {}", di));
    debug::write(stdout, &format!("x0 {}", y0));
    debug::write(stdout, "******");
    debug::write(stdout, &format!("y0 {} y1 {}", y0, y1));
    debug::write(stdout, &format!("y0_f {}, y1_f {}", y0_f, y1_f));
    debug::write(stdout, &format!("y0_i {}, y1_i {}", y0_i, y1_i));
    debug::write(stdout, "PLOT_HIGH------------");

    // decimal place of x
    let dec_y = y0 - y0_f;

    let mut x = x0 + di * dec_y;

    let mut prev_vec = p0;

    let mut y = y0_i;
    let mut end = false;

    loop {
        if y == y1_i {
            // if incrementing and x greater than x1, break
            end = true
        }

        // debug::write(stdout, "In Loop");

        let x_ceil = x.ceil();
        let x_floor = x.floor();

        let map_ceil = map.get(&(x_ceil as u16, y as u16));
        let map_floor = map.get(&(x_floor as u16, y as u16));

        if map_ceil.is_some() {
            debug::write(stdout, &format!("({}, {}) 'W' ceil", x_ceil, y));
            debug::write(stdout, &format!("prev_vec {}", prev_vec));
            return prev_vec;
        } else if map_floor.is_some() {
            debug::write(stdout, &format!("({}, {}) 'W' floor", x_floor, y));
            debug::write(stdout, &format!("prev_vec {}", prev_vec));
            return prev_vec;
        } else {
            debug::write(stdout, &format!("({}, {}) ' ' ceil", x_ceil, y));
            debug::write(stdout, &format!("({}, {}) ' ' floor", x_floor, y));
        }

        if end {
            break;
        }

        prev_vec = Vector { x: x, y: y as f32 };

        x += dx;
        y += yi;
    }

    return p1;
}

pub fn plot_line(
    stdout: &mut impl Write,
    p0: Vector<f32>,
    p1: Vector<f32>,
    map: &MapData,
) -> Vector<f32> {
    let Vector { x: x0, y: y0 } = p0;
    let Vector { x: x1, y: y1 } = p1;

    if p0 == p1 {
        return p0;
    }

    if (y1 - y0).abs() < (x1 - x0).abs() {
        plot_line_low(stdout, p0, p1, map)
    //     if x0 > x1 {
    //         plot_line_low(p1, p0)
    //     } else {
    //         plot_line_low(p0, p1)
    //     }
    } else {
        plot_line_high(stdout, p0, p1, map)
    }
}
