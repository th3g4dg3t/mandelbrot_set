use std::path::{Path, PathBuf};

use clap::Parser;

#[derive(Clone, Copy)]
pub struct Rectangle(pub f64, pub f64, pub f64);

#[derive(Clone, Copy)]
pub struct Resolution(pub u32, pub u32);

// Helper function to parse "X,Y,LENGTH" or "X:Y:LENGTH"
fn parse_rectangle(s: &str) -> Result<Rectangle, String> {
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

    Ok(Rectangle(x, y, len))
}

// Helper function to parse "X,Y" or "X:Y"
fn parse_resolution(s: &str) -> Result<Resolution, String> {
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

    Ok(Resolution(x, y))
}

#[derive(Parser)]
pub struct Cli {
    /// Output image path
    #[arg(value_name = "PATH")]
    output_path: PathBuf,

    /// Image resolution
    #[arg(short, long, value_parser = parse_resolution, value_name = "X,Y")]
    resolution: Option<Resolution>,

    /// Rectangle parameters as "X,Y,LENGTH" or "X:Y:LENGTH", where LENGTH is the length of the vertical sides and (X,Y) are the coordinates of the top left corner
    #[arg(short = 'R',
	  long,
	  value_parser = parse_rectangle,
	  value_name = "X,Y,LENGTH",
	  allow_hyphen_values = true
    )]
    rectangle: Option<Rectangle>,
}

impl Cli {
    pub fn output_path(&self) -> &Path {
        &self.output_path
    }

    pub fn output_path_mut(&mut self) -> &mut PathBuf {
        &mut self.output_path
    }

    pub fn resolution(&self) -> Option<Resolution> {
        self.resolution
    }

    pub fn rectangle(&self) -> Option<Rectangle> {
        self.rectangle
    }
}
