// bresenham line algorithm implementation
// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm

fn plot_line_low((x0, y0): (i16, i16), (x1, y1): (i16, i16)) -> Vec<(i16, i16)> {
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
        if d > 0 {
            y = y + yi;
            d = d - 2 * dx;
        }

        d = d + 2 * dy;
    }

    line_vec
}

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

pub fn plot_line(p0: (i16, i16), p1: (i16, i16)) -> Vec<(i16, i16)> {
    let (x0, y0) = p0;
    let (x1, y1) = p1;
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
