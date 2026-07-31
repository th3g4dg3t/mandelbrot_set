use anyhow::Result;
use clap::Parser;
use image::{ImageBuffer, Rgb};
use num_complex::Complex64;
use rayon::prelude::*;

mod cli;

fn main() -> Result<()> {
    let mut user_input = cli::Cli::parse();
    user_input.output_path_mut().set_extension("png");

    // Define the image size and the part of the complex plain to display
    let cli::Resolution(imgx, imgy) = user_input.resolution().unwrap_or(cli::Resolution(800, 800));
    let cli::Rectangle(x_0, y_0, side) = user_input
        .rectangle()
        .unwrap_or(cli::Rectangle(-2.0, 2.0, 4.0));

    // Step sizes to define a grid of complex number and of colors
    let step = side / imgy as f64;
    let color_step_x = 255.0 / imgx as f64;
    let color_step_y = 255.0 / imgy as f64;

    // This is where the image will be stored in memory as a bitmap
    let mut image_buf = ImageBuffer::new(imgx, imgy);

    // For each pixel compute whether the corresponding complex number is
    // in the Mandelbrot set or not using a parallel iterator
    image_buf
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            let re = x_0 + step * x as f64;
            let im = y_0 - step * y as f64;
            let c = Complex64::new(re, im);

            let mut g = 0;
            let mut z = Complex64::new(0.0, 0.0);
            while g < 255 && z.norm() <= 2.0 {
                z = z.powu(2) + c;
                g += 1;
            }

            let r = (color_step_x * x as f64) as u8;
            let b = (color_step_y * y as f64) as u8;

            *pixel = Rgb::<u8>([r, g, b]);
        });

    image_buf.save(user_input.output_path())?;

    Ok(())
}
