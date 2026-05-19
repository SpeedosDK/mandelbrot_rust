use image::{RgbImage, Rgb};
use rayon::prelude::*;

pub fn render(width: u32, height: u32) -> RgbImage {
    let max_iter = 1000;

    // Paralleliser over rækker og generer hver række i en lokal buffer
    let rows: Vec<Vec<Rgb<u8>>> = (0..height)
        .into_par_iter()
        .map(|y| {
            let mut row = Vec::with_capacity(width as usize);

            for x in 0..width {
                let (zx, zy, iter) = crate::utils::mandelbrot_point(x, y, width, height, max_iter);
                let color = crate::utils::colorize(zx, zy, iter, max_iter);
                row.push(color);
            }

            row
        })
        .collect();
    // Merge rækkerne ned i billedet (sekventielt og trådsikkert)
    let mut img = RgbImage::new(width, height);
    for(y, row) in rows.into_iter().enumerate() {
        for(x, pixel) in row.into_iter().enumerate() {
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }
    img
}