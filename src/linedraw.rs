use crate::vectors::Vector;

// bresenham line algorithm implementation
// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm

// fn plot_line_low((x0, y0): (i16, i16), (x1, y1): (i16, i16)) -> Vec<(i16, i16)> {
fn plot_line_low(p0: Vector<f32>, p1: Vector<f32>) -> Vec<(i16, i16)> {
    let Vector { x: x0, y: y0 } = p0.round_i_int();
    let Vector { x: x1, y: y1 } = p1.round_i_int();

    let extra = if p0.y.floor() != p1.y.ceil() {
        true
    } else {
        false
    };

    let (dx, mut dy) = (x1 - x0, y1 - y0);
    let mut yi = 1;

    if dy < 0 {
        yi = -1;
        dy = -1 * dy;
    }

    let mut d = 2 * dy - dx;
    let mut y = y0;

    let mut line_vec = Vec::new();

    for x in x0..x1 {
        // plot x,y
        line_vec.push((x, y));
        if extra {
            line_vec.push((x, y + 1));
        }
        if d > 0 {
            y = y + yi;
            d = d - 2 * dx;
        }

        d = d + 2 * dy;
    }

    line_vec
}

// fn plot_line_low(prev_point: Vector<f32>, point: Vector<f32>, map: Map) -> Vector<f32> {
//     let Vector {
//         x: diff_x,
//         y: diff_y,
//     } = point - prev_point;
//
//     let mut ratio: f32;
//     let mut inc_vec: Vector<f32>;
//
//     let ratio = diff_y / diff_x; // ratio of y / x
//
//     // if diff_x == 0.0 {}
//
//     // vector to increment check by
//     let inc_vec = if prev_point.x < point.x {
//         Vector { x: 1.0, y: ratio }
//     } else {
//         Vector {
//             x: -1.0,
//             y: -1.0 * ratio,
//         }
//     };
//
//     let mut prev_new_point = prev_point;
//     let mut new_point = prev_point;
//
//     let mut index = 0;
//
//     loop {
//         if prev_point.x < point.x && new_point.x >= point.x {
//             prev_point = point;
//             break;
//         }
//         if prev_point.x > point.x && new_point.x <= point.x {
//             prev_point = point;
//             break;
//         }
//
//         let x_index = new_point.x as u16;
//
//         let y_ceil = new_point.y.ceil() as u16;
//         let y_floor = new_point.y.floor() as u16;
//
//         index += 1;
//
//         let result_floor = map.get(&(x_index, y_floor));
//         let result_ceil = map.get(&(x_index, y_ceil));
//
//         if result_floor.is_some() || result_ceil.is_some() {
//             self.point = prev_new_point;
//             self.point.x = self.point.x.round();
//             self.prev_point = self.point;
//             break;
//         }
//
//         prev_new_point = new_point;
//         new_point += inc_vec;
// }

fn plot_line_high((x0, y0): (i16, i16), (x1, y1): (i16, i16)) -> Vec<(i16, i16)> {
    let (mut dx, dy) = (x1 - x0, y1 - y0);
    let mut xi = 1;

    if dx < 0 {
        xi = -1;
        dx = -1 * dx;
    }

    let mut d = 2 * dx - dy;
    let mut x = x0;

    let mut line_vec = Vec::new();

    for y in y0..y1 {
        // plot x,y
        line_vec.push((x, y));
        if d > 0 {
            x = x + xi;
            d = d - 2 * dy;
        }

        d = d + 2 * dx;
    }

    line_vec
}

pub fn plot_line(p0: Vector<f32>, p1: Vector<f32>) -> Vec<(i16, i16)> {
    let Vector { x: x0, y: y0 } = p0;
    let Vector { x: x1, y: y1 } = p1;

    if (y1 - y0).abs() < (x1 - x0).abs() {
        if x0 > x1 {
            plot_line_low(p1, p0)
        } else {
            plot_line_low(p0, p1)
        }
    } else {
        if y0 > y1 {
            plot_line_high(p1, p0)
        } else {
            plot_line_high(p0, p1)
        }
    }
}
