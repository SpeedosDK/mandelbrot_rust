use image::{Rgb, RgbImage};


pub fn render(width: u32, height: u32) -> RgbImage {
    let max_iter = 1000;
    let mut img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let (zx, zy, iter) = crate::utils::mandelbrot_point(x, y, width, height, max_iter);
            let color = crate::utils::colorize(zx, zy, iter, max_iter);
            img.put_pixel(x, y, color);
        }
    }
    img
}