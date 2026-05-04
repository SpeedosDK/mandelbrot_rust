use image::Rgb;

pub fn mandelbrot_point(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    max_iter: u32,
) -> (f64, f64, u32) {
    let cx = (x as f64 - width as f64 / 2.0) * 4.0 / width as f64;
    let cy = (y as f64 - height as f64 / 2.0) * 4.0 / height as f64;

    let mut zx = 0.0;
    let mut zy = 0.0;
    let mut iter = 0;

    while zx * zx + zy * zy < 4.0 && iter < max_iter {
        let tmp = zx * zx - zy * zy + cx;
        zy = 2.0 * zx * zy + cy;
        zx = tmp;
        iter += 1;
    }
    (zx, zy, iter)
}

pub fn colorize(zx: f64, zy: f64, iter: u32, max_iter: u32) -> Rgb<u8> {
    if iter == max_iter {
        return Rgb([0, 0, 0])
    }
    let mag = zx * zx + zy * zy;
    let smooth = if mag > 1.0 {
        iter as f64 + 1.0 - (mag.ln() / 2_f64.ln())
    } else {
        iter as f64
    };

    let t = smooth / max_iter as f64;
    let r = (0.5 + 0.5 * (6.28318 * t).cos()) * 255.0;
    let g = (0.5 + 0.5 * (6.28318 * t + 2.094).cos()) * 255.0;
    let b = (0.5 + 0.5 * (6.28318 * t + 4.189).cos()) * 255.0;

    Rgb([r as u8, g as u8, b as u8])
}
