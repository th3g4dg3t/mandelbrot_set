use anyhow::{Context, Result};
use clap::Parser;
use image::{ImageFormat, RgbImage};
use rayon::prelude::*;

mod cli;

fn main() -> Result<()> {
    let mut user_input = cli::Cli::parse();

    // Set the default image format as PNG if unspecified, not supported or not recognised
    if let Some(ext) = user_input.output_path().extension() {
        if let Some(image_format) = ImageFormat::from_extension(ext) {
            if !image_format.can_write() {
                eprintln!("The chosen format is not supported. Choosing PNG instead.");
                user_input.output_path_mut().set_extension("png");
            }
        } else {
            eprintln!(
                "Unrecognised format \"{}\". Using PNG instead.",
                ext.to_string_lossy()
            );
            user_input.output_path_mut().set_extension("png");
        };
    } else {
        user_input.output_path_mut().set_extension("png");
    }

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
    let mut container = vec![0; imgx as usize * imgy as usize * 3];

    // For each pixel compute whether the corresponding complex number is
    // in the Mandelbrot set or not using a parallel iterator
    container
        .par_chunks_exact_mut(imgx as usize * 3)
        .enumerate()
        .for_each(|(y, row)| {
            let c_im = y_0 - step * y as f64;
            let b = (color_step_y * y as f64) as u8;
            row.chunks_exact_mut(3).enumerate().for_each(|(x, pixel)| {
                let c_re = x_0 + step * x as f64;

                let mut z_re = 0.0_f64;
                let mut z_im = 0.0_f64;

                let mut z_re_sq = 0.0_f64;
                let mut z_im_sq = 0.0_f64;

                let mut g = 0;
                while g < 255 && z_re_sq + z_im_sq <= 4.0 {
                    z_im = (2.0_f64 * z_re).mul_add(z_im, c_im);
                    z_re = z_re_sq - z_im_sq + c_re;

                    z_re_sq = z_re * z_re;
                    z_im_sq = z_im * z_im;

                    g += 1;
                }

                let r = (color_step_x * x as f64) as u8;

                pixel[0] = r;
                pixel[1] = g;
                pixel[2] = b;
            });
        });

    // Take the computed vector and treat it as an image. This should not fail ever.
    let image_buf = RgbImage::from_raw(imgx, imgy, container)
        .context("Failed to convert the container into an RGB image.")?;

    image_buf.save(user_input.output_path())?;

    Ok(())
}
