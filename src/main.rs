mod color_placer;
mod colors;
mod errors;
#[allow(unused)]
#[allow(dead_code)]
mod grid;
mod math;
mod pixel_fitter;
mod scratch;
mod stopwatch;
mod traits;

use crate::color_placer::ColorPlacer;
use crate::colors::*;
use crate::grid::*;
use crate::pixel_fitter::*;
use crate::stopwatch::Stopwatch;
use crate::traits::*;
use bmp::Image;
use chrono::*;
use rand::prelude::*;
use rand::rngs::StdRng;
use rustc_hash::FxHashSet;

fn main() {
    // timing
    let mut stopwatch = Stopwatch::start_new();

    //let grid_size = Size::new(1584, 396);
    let grid_size = Size::new(256, 256);

    //let min_max = ColorChannelDepths::find_min_max_depths(1584, 396);
    let (red, green, blue) = ColorChannel::find_min_mid_max_depths(grid_size).unwrap_or_default();

    let red_channel = ColorChannel::spaced(red);
    let green_channel = ColorChannel::spaced(green);
    let blue_channel = ColorChannel::spaced(blue);

    let colorspace = ColorSpaceGenerator::new(red_channel, green_channel, blue_channel);

    let mut colors = colorspace.get_colors();

    const SEED: u64 = 147;
    let mut sorter = FnColorSorter::new(|colors| {
        let mut rand = StdRng::seed_from_u64(SEED);
        colors.shuffle(&mut rand);
        colors.sort_by_key(|color| {
            // demon fire
            //((color.red) << 16) + ((color.blue) << 8) + (color.green)
            //((color.red) << 16) + (color.blue) + (color.green)

            // white -> blue/black
            let dist_from_white = Color::WHITE.dist(color);
            let dist_from_blue = Color::MAX_DIST - color.dist(&Color::new(0, 0, 255));
            dist_from_white + dist_from_blue

            //(color.blue * color.blue) + color.red + color.green

            /*(255 + 255 + 255) -*/
            //(color.red) + (color.blue) + (color.green)

            // let min = color.red.min(color.green.min(color.blue));
            // let max = color.red.max(color.green.max(color.blue));
            // max - min

            //((color.red + color.green + color.blue) as f32 / 3.0) as usize

            // usize::abs_diff(color.red, color.green)
            //     + usize::abs_diff(color.red, color.blue)
            //     + usize::abs_diff(color.green, color.blue)
        });
    });
    sorter.sort_colors(&mut colors);

    println!("Color Setup: {:?}", stopwatch.restart_elapsed());

    let mut grid = Grid::new(grid_size);
    NeighborManager::set_neighbors(&mut grid, NeighborManager::standard_offsets(), false);

    let mut available = FxHashSet::default();
    // starting position
    let start_pos = Point::new(
        grid_size.width / 8,
        grid_size.height - (grid_size.height / 30),
    );
    available.insert(start_pos);

    let fitter = ColorDistPixelFitter;

    println!(
        "Grid, Neighbors, and Available Setup: {:?}",
        stopwatch.restart_elapsed()
    );

    ColorPlacer::fill_grid::<ColorDistPixelFitter>(&mut grid, &mut available, &colors, &fitter);

    let image: Image = grid.try_into().expect("missing pixel!");

    let now = Local::now();

    let file_name = format!("rust_{}.bmp", now.format("%Y%m%d_%H%M%S"));

    let path = format!("c:\\temp\\all_colors\\{}", file_name);
    let saved = image.save(path);
    saved.expect("Should have saved");
}
