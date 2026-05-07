use std::path::PathBuf;

use clap::Parser;
use wherewasi::{build_density_grid, parse_my_data, render_heatmap};

#[derive(Debug, clap::Parser)]
pub struct Options {
    #[arg(short, long)]
    input_file: String,
    #[arg(short, long, default_value_t = 5.0)]
    blur_sigma: f32,
    #[arg(short, long, default_value_t = 800)]
    width: usize,
    #[arg(short, long, default_value_t = 600)]
    height: usize,
    #[arg(short, long, default_value_t = String::from("heatmap.png"))]
    output_file: String,
}

fn main() {
    let options = Options::parse();
    println!("Input file: {}", options.input_file);

    let coords = parse_my_data(&options.input_file).expect("Failed to parse data");
    println!("Parsed coordinates has {} elements", coords.len());
    if coords.is_empty() {
        println!("No coordinates found in the input data. Exiting.");
        return;
    }

    let grid = build_density_grid(&coords);
    println!("Built density grid with dimensions: {}x{}", grid.width(), grid.height());

    let image = render_heatmap(grid, options.blur_sigma);
    let output_path = PathBuf::from(&options.output_file);
    image.save(&output_path).expect("Failed to save heatmap image");
    println!("Heatmap saved to: {}", output_path.display());
}
