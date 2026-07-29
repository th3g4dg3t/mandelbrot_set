use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use image::{ImageBuffer, Rgb};
use num_complex::Complex64;
use rayon::prelude::*;

// Helper function to parse "X,Y,LENGTH" or "X:Y:LENGTH"
fn parse_rectangle(s: &str) -> Result<(f64, f64, f64), String> {
    let parts: Vec<_> = s.split([',', ':']).collect();

    if parts.len() != 3 {
        return Err(format!(
            "expected 3 values separated by ',' or ':', but found {}",
            parts.len()
        ));
    }

    let x = parts[0]
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("invalid number for X: '{}'", parts[0]))?;
    let y = parts[1]
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("invalid number for Y: '{}'", parts[1]))?;
    let len = parts[2]
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("invalid number for LENGTH: '{}'", parts[2]))?;

    Ok((x, y, len))
}

// Helper function to parse "X,Y" or "X:Y"
fn parse_resolution(s: &str) -> Result<(u32, u32), String> {
    let parts: Vec<_> = s.split([',', ':']).collect();

    if parts.len() != 2 {
        return Err(format!(
            "expected 2 values separated by ',' or ':', but found {}",
            parts.len()
        ));
    }

    let x = parts[0]
        .trim()
        .parse::<u32>()
        .map_err(|_| format!("invalid number for X: '{}'", parts[0]))?;
    let y = parts[1]
        .trim()
        .parse::<u32>()
        .map_err(|_| format!("invalid number for Y: '{}'", parts[1]))?;

    Ok((x, y))
}

#[derive(Parser)]
struct Cli {
    /// Output image path
    #[arg(value_name = "PATH")]
    output_path: PathBuf,

    /// Image resolution
    #[arg(short, long, value_parser = parse_resolution, value_name = "X,Y")]
    resolution: Option<(u32, u32)>,

    /// Rectangle parameters as "X,Y,LENGTH" or "X:Y:LENGTH", where LENGTH is the length of the vertical sides and (X,Y) are the coordinates of the top left corner
    #[arg(short = 'R',
	  long,
	  value_parser = parse_rectangle,
	  value_name = "X,Y,LENGTH",
	  allow_hyphen_values = true
    )]
    rectangle: Option<(f64, f64, f64)>,
}

fn main() -> Result<()> {
    let Cli {
        mut output_path,
        resolution,
        rectangle,
    } = Cli::parse();
    output_path.set_extension("png");

    // Define the image size and the part of the complex plain to display
    let (imgx, imgy) = resolution.unwrap_or((800, 800));
    let (x_0, y_0, side) = rectangle.unwrap_or((-2.0, 2.0, 4.0));

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

    image_buf.save(&output_path)?;

    Ok(())
}
