pub mod model;

use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use image::{ImageBuffer, Luma, Rgba, RgbaImage};
use imageproc::filter::gaussian_blur_f32;

use crate::model::Data;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

pub fn parse_my_data(filepath: &str) -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
    // 1. Open file and wrap in a buffered reader for performance
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    // 2. Stream deserialize the JSON payload
    let data: Data = serde_json::from_reader(reader)?;
    
    // Pre-allocate to avoid reallocation overhead. 
    // A typical year of data might have ~100k-500k points.
    let mut points = Vec::with_capacity(100_000);

    // 3. Extract and parse coordinates
    for movement in data {
        // Extract granular movement path
        for path in movement.timeline_path {
            if let Some(coords) = parse_geo_string(&path.point) {
                points.push(coords);
            }
        }

        // Extract static visit locations (to ensure destinations are represented)
        if let Some(visit) = movement.visit {
            if let Some(coords) = parse_geo_string(&visit.top_candidate.place_location) {
                points.push(coords);
            }
        }
    }

    // Shrink the vector to drop excess capacity
    points.shrink_to_fit();

    Ok(points)
}

/// Fast parser for Google's "geo:LAT,LON" string format
#[inline]
fn parse_geo_string(geo_str: &str) -> Option<(f64, f64)> {
    // Remove the "geo:" prefix if it exists
    let clean_str = geo_str.strip_prefix("geo:").unwrap_or(geo_str);
    
    // Split once at the comma
    let (lat_str, lon_str) = clean_str.split_once(',')?;

    // Parse both halves directly to f64
    let lat: f64 = lat_str.parse().ok()?;
    let lon: f64 = lon_str.parse().ok()?;

    Some((lat, lon))
}

pub fn build_density_grid(coords: &[(f64, f64)]) -> ImageBuffer<Luma<f32>, Vec<f32>> {
    // 1. Find Bounding Box
    let mut min_lat = f64::MAX;
    let mut max_lat = f64::MIN;
    let mut min_lon = f64::MAX;
    let mut max_lon = f64::MIN;

    for &(lat, lon) in coords {
        if lat < min_lat { min_lat = lat; }
        if lat > max_lat { max_lat = lat; }
        if lon < min_lon { min_lon = lon; }
        if lon > max_lon { max_lon = lon; }
    }

    // 2. Create the Accumulator Grid (Raw Density)
    // We use an ImageBuffer of f32 to allow for Gaussian blurring later.
    let mut grid = ImageBuffer::from_pixel(WIDTH as u32, HEIGHT as u32, Luma([0.0f32]));

    let lat_range = max_lat - min_lat;
    let lon_range = max_lon - min_lon;

    // 3. Accumulate Points
    for &(lat, lon) in coords {
        // Normalize coordinates to grid dimensions
        let x = (((lon - min_lon) / lon_range) * (WIDTH as f64 - 1.0)) as u32;
        // Invert Y so North is "up" in the final image
        let y = (((max_lat - lat) / lat_range) * (HEIGHT as f64 - 1.0)) as u32;

        let pixel = grid.get_pixel_mut(x, y);
        pixel.0[0] += 1.0; 
    }

    grid
}

pub fn render_heatmap(density_grid: ImageBuffer<Luma<f32>, Vec<f32>>, blur_sigma: f32) -> RgbaImage {
    // 1. Apply Gaussian Blur (The KDE equivalent)
    let blurred_grid = gaussian_blur_f32(&density_grid, blur_sigma);

    // 2. Find Max Density for Normalization
    let mut max_density = 0.0f32;
    for pixel in blurred_grid.pixels() {
        if pixel.0[0] > max_density {
            max_density = pixel.0[0];
        }
    }

    // 3. Render to Color
    let mut output_image = RgbaImage::new(blurred_grid.width(), blurred_grid.height());

    for (x, y, pixel) in blurred_grid.enumerate_pixels() {
        let density = pixel.0[0];
        let normalized = if max_density > 0.0 { density / max_density } else { 0.0 };

        // Apply a color gradient based on normalized density [0.0, 1.0]
        let color = density_to_color(normalized);
        output_image.put_pixel(x, y, color);
    }

    output_image
}

/// Simple cold-to-hot gradient (Blue -> Cyan -> Yellow -> Red)
fn density_to_color(t: f32) -> Rgba<u8> {
    // Apply a power function (e.g., t.sqrt() or t.powf(0.5)) if low-density 
    // points are too dim compared to massive clusters like your home.
    let t = t.powf(0.5); 

    let r = (255.0 * t.min(1.0)) as u8;
    let g = (255.0 * (t * 2.0 - 1.0).max(0.0)) as u8;
    let b = (255.0 * (1.0 - t).max(0.0)) as u8;
    let a = if t > 0.01 { 255 } else { 0 }; // Make completely empty space transparent

    Rgba([r, g, b, a])
}