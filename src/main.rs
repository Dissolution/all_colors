#[allow(unused)]
#[allow(dead_code)]
mod color_placer;
mod colors;
mod errors;
mod grid;
mod math;
mod pixel_fitter;
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
    let pixel_count = grid_size.area();
    //let min_max = ColorChannelDepths::find_min_max_depths(1584, 396);

    // let (green, red, blue) = ColorChannel::find_min_mid_max_depths(grid_size).unwrap();
    // let red_channel = ColorChannel::spaced(red);
    // let green_channel = ColorChannel::spaced(green);
    // let blue_channel = ColorChannel::spaced(blue);

    // let (min, max) = ColorChannel::find_min_max_depths(grid_size).unwrap();
    // let red_channel = ColorChannel::spaced(min);
    // let green_channel = ColorChannel::spaced(min);
    // let blue_channel = ColorChannel::spaced(max);
    //
    // let colorspace = ColorSpaceGenerator::new(red_channel, green_channel, blue_channel);
    // let mut colors = colorspace.get_colors();

    let mut colors = Vec::with_capacity(pixel_count);
    let mut i: isize = 0;
    while colors.len() < pixel_count {
        colors.push(Color::new(i as u8, 0, 0));
        colors.push(Color::new(0, i as u8, 0));
        colors.push(Color::new(0, 0, i as u8));
        i += 1;
        if i > (u8::MAX as isize) {
            i = 0;
        }
    }

    while colors.len() > pixel_count {
        colors.remove(colors.len() - 1);
    }

    //
    // for _ in 0..pixel_count {
    //     let color = Color::new(0, 0, r as u8);
    //     colors.push(color);
    //     r += 1;
    //     if r > u8::MAX as isize {
    //         r = 0;
    //     }
    // }

    assert_eq!(colors.len(), pixel_count);

    const SEED: u64 = 147;
    let mut sorter = FnColorSorter::new(|colors| {
        let mut rand = StdRng::seed_from_u64(SEED);
        colors.shuffle(&mut rand);
        // colors.sort_by_key(|color| {
        //     // demon fire
        //     //((color.red as usize) << 16) + ((color.blue as usize) << 8) + (color.green as usize)
        //     //((color.red) << 16) + (color.blue) + (color.green)
        //
        //     // white -> blue/black
        //     let dist_from_white = Color::WHITE.dist(color);
        //     let dist_from_black = Color::BLACK.dist(color);
        //     let dist_from_blue = /* Color::MAX_DIST - */ color.dist(&Color::new(0, 0, 255));
        //     dist_from_black + dist_from_blue
        //
        //     //(color.blue * color.blue) + color.red + color.green
        //
        //     /*(255 + 255 + 255) -*/
        //     //(color.red) + (color.blue) + (color.green)
        //
        //     // let min = color.red.min(color.green.min(color.blue));
        //     // let max = color.red.max(color.green.max(color.blue));
        //     // max - min
        //
        //     //((color.red + color.green + color.blue) as f32 / 3.0) as usize
        //
        //     // usize::abs_diff(color.red, color.green)
        //     //     + usize::abs_diff(color.red, color.blue)
        //     //     + usize::abs_diff(color.green, color.blue)
        // });
    });
    sorter.sort_colors(&mut colors);

    println!("Color Setup: {:?}", stopwatch.restart_elapsed());

    let mut grid = Grid::new(grid_size);
    NeighborManager::set_neighbors(&mut grid, &NeighborManager::STD_OFFSETS, true);

    let mut start_points = FxHashSet::default();

    /* Starting Position(s) */
    // let start_pos = Point::new(
    //     grid_size.width / 8,
    //     grid_size.height - (grid_size.height / 30),
    // );
    let center_pos = Point::new(grid_size.width / 2, grid_size.height / 2);
    // available.insert(start_pos);

    let mut rand = StdRng::seed_from_u64(SEED);

    // for _ in 0..16 {
    //     let x = rand.gen_range(0..grid_size.width);
    //     let y = rand.gen_range(0..grid_size.height);
    //     let pt = Point::new(x, y);
    //     start_points.insert(pt);
    // }
    start_points.insert(center_pos);

    //let fitter = ColorAndPixelDistPixelFitter::new(center_pos);
    let fitter = ColorDistPixelFitter;

    println!(
        "Grid, Neighbors, Available, and Fitter Setup: {:?}",
        stopwatch.restart_elapsed()
    );

    let start_points: Vec<Point> = start_points.into_iter().collect();

    ColorPlacer::fill_grid(&mut grid, start_points.as_slice(), &colors, &fitter);

    println!("Grid Filled: {:?}", stopwatch.restart_elapsed());

    let image: Image = grid.try_into().expect("missing pixel!");

    let now = Local::now();

    let file_name = format!("rust_{}.bmp", now.format("%Y%m%d_%H%M%S"));

    let path = format!("c:\\temp\\all_colors\\{}", file_name);
    let saved = image.save(path);
    saved.expect("Should have saved");

    println!("Image created and saved: {:?}", stopwatch.restart_elapsed());
}
