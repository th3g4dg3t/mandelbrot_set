use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use image::{ImageBuffer, Rgb};
use num_complex::Complex64;
use rayon::prelude::*;

// Helper function to parse "X,Y,LENGTH" or "X:Y:LENGTH"
fn parse_square(s: &str) -> Result<(f64, f64, f64), String> {
    // Split on either comma or colon
    let parts: Vec<&str> = s.split([',', ':']).collect();

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

#[derive(Parser)]
struct Cli {
    /// Output image path
    #[arg(value_name = "NAME")]
    output_path: PathBuf,

    /// Image resolution
    #[arg(short, long)]
    resolution: Option<u32>,

    /// Square parameters as "X,Y,LENGTH" or "X:Y:LENGTH"
    #[arg(short, long, value_parser = parse_square, value_name = "X,Y,LENGTH")]
    square: Option<(f64, f64, f64)>,
}

fn main() -> Result<()> {
    let Cli {
        mut output_path,
        resolution,
        square,
    } = Cli::parse();
    output_path.set_extension("png");

    let resolution = resolution.unwrap_or(800);

    let mut image_buf = ImageBuffer::new(resolution, resolution);
    image_buf
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            *pixel = Rgb::<u8>([0, 0, 0]);
        });

    image_buf.save(&output_path)?;

    Ok(())
}
