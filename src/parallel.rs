use image::{RgbImage,  Rgb};
use rayon::prelude::*;

pub fn render(width: u32, height: u32) -> RgbImage {
    let max_iter = 1000;

    let mut img = RgbImage::new(width, height);

    img.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let (zx, zy, iter) = crate::sequential::mandelbrot_point(x, y, width, height, max_iter);
            *pixel = crate::sequential::colorize(zx, zy, iter, max_iter);
        });
    img
}